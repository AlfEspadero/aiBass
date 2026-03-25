use defmt::{info, warn};
use embassy_executor::task;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Receiver, Sender};

pub const SAMPLE_RATE_HZ: u32 = 1_000;
pub const SAMPLE_WINDOW_LEN: usize = 128;

pub type PitchInputReceiver = Receiver<'static, ThreadModeRawMutex, PitchInput, 4>;
pub type UiStateSender = Sender<'static, ThreadModeRawMutex, crate::ui::UiState, 4>;

#[derive(Copy, Clone)]
pub struct PitchInput {
    pub values: [i32; SAMPLE_WINDOW_LEN],
}

#[derive(Copy, Clone)]
pub struct PitchEstimate {
    pub frequency_hz_x100: u32,
    pub confidence_permille: u16,
}

const NOTE_NAMES: [&str; 4] = ["E1", "A1", "D2", "G2"];
const NOTE_FREQ_HZ_X100: [u32; 4] = [4120, 5500, 7342, 9800];
const MIN_LAG: usize = 8;
const MAX_LAG: usize = 32;

fn estimate_pitch(input: &PitchInput) -> Option<PitchEstimate> {
    let mut best_lag = 0usize;
    let mut best_corr: i64 = i64::MIN;
    let mut best_energy: i64 = 1;

    for lag in MIN_LAG..=MAX_LAG {
        let mut corr: i64 = 0;
        let mut e1: i64 = 0;
        let mut e2: i64 = 0;

        let n = SAMPLE_WINDOW_LEN - lag;
        for i in 0..n {
            let a = input.values[i] as i64;
            let b = input.values[i + lag] as i64;
            corr += a * b;
            e1 += a * a;
            e2 += b * b;
        }

        let energy = ((e1 + e2) / 2).max(1);
        if corr > best_corr {
            best_corr = corr;
            best_lag = lag;
            best_energy = energy;
        }
    }

    if best_lag == 0 || best_corr <= 0 {
        return None;
    }

    let confidence_permille = ((best_corr * 1000) / best_energy).clamp(0, 1000) as u16;
    if confidence_permille < 220 {
        return None;
    }

    let freq_x100 = (SAMPLE_RATE_HZ as u64 * 100 / best_lag as u64) as u32;
    Some(PitchEstimate {
        frequency_hz_x100: freq_x100,
        confidence_permille,
    })
}

fn nearest_bass_note(freq_hz_x100: u32) -> (usize, i32) {
    let mut best_idx = 0usize;
    let mut best_abs = i64::MAX;
    let mut signed_delta = 0i32;

    for (idx, target) in NOTE_FREQ_HZ_X100.iter().enumerate() {
        let delta = freq_hz_x100 as i64 - *target as i64;
        let abs = delta.abs();
        if abs < best_abs {
            best_abs = abs;
            best_idx = idx;
            signed_delta = delta as i32;
        }
    }

    (best_idx, signed_delta)
}

fn approx_cents_x10(freq_hz_x100: u32, target_hz_x100: u32) -> i32 {
    // Small-error linear approximation around the target:
    // cents ~= 1731.234 * (f/ft - 1)
    let ratio_err = (freq_hz_x100 as f32 / target_hz_x100 as f32) - 1.0;
    (ratio_err * 17312.34) as i32
}

fn make_square_like(freq_hz_x100: u32) -> PitchInput {
    let period_samples = ((SAMPLE_RATE_HZ as u64 * 100) / freq_hz_x100 as u64).max(2) as usize;
    let half = (period_samples / 2).max(1);
    let mut values = [0i32; SAMPLE_WINDOW_LEN];

    for (i, v) in values.iter_mut().enumerate() {
        let phase = i % period_samples;
        *v = if phase < half { 12_000 } else { -12_000 };
    }

    PitchInput { values }
}

pub fn run_validation_harness() {
    let mut pass = 0u8;
    for (idx, target) in NOTE_FREQ_HZ_X100.iter().enumerate() {
        let input = make_square_like(*target);
        let Some(est) = estimate_pitch(&input) else {
            warn!("validation {} failed: no estimate", NOTE_NAMES[idx]);
            continue;
        };

        let (note_idx, _) = nearest_bass_note(est.frequency_hz_x100);
        let freq_err = est.frequency_hz_x100.abs_diff(*target);
        if note_idx == idx && freq_err <= 600 {
            pass += 1;
            info!(
                "validation {} ok freq={}cHz conf={}",
                NOTE_NAMES[idx],
                est.frequency_hz_x100,
                est.confidence_permille
            );
        } else {
            warn!(
                "validation {} failed freq={}cHz note_idx={}",
                NOTE_NAMES[idx],
                est.frequency_hz_x100,
                note_idx
            );
        }
    }

    info!("validation summary: {}/{} vectors passed", pass, NOTE_NAMES.len());
}

#[task]
pub async fn pitch_task(rx: PitchInputReceiver, ui_tx: UiStateSender) {
    let mut stable_hits: u8 = 0;
    let mut miss_hits: u8 = 0;

    loop {
        let input = rx.receive().await;
        let Some(estimate) = estimate_pitch(&input) else {
            stable_hits = 0;
            miss_hits = miss_hits.saturating_add(1);
            if miss_hits >= 3 {
                let _ = ui_tx.try_send(crate::ui::UiState::no_pitch());
            }
            continue;
        };
        miss_hits = 0;
        stable_hits = stable_hits.saturating_add(1);

        let (idx, _delta) = nearest_bass_note(estimate.frequency_hz_x100);
        let target = NOTE_FREQ_HZ_X100[idx];
        let cents_x10 = approx_cents_x10(estimate.frequency_hz_x100, target);
        let state = crate::ui::UiState::from_pitch(
            NOTE_NAMES[idx],
            estimate.frequency_hz_x100,
            cents_x10,
            estimate.confidence_permille,
        );

        // Require a short confirmation streak before surfacing pitch updates.
        if stable_hits < 2 {
            continue;
        }

        if ui_tx.try_send(state).is_err() {
            warn!("ui queue full; dropping pitch state");
        } else {
            info!(
                "pitch freq={}cHz note={} centsx10={} conf={}",
                estimate.frequency_hz_x100,
                NOTE_NAMES[idx],
                cents_x10,
                estimate.confidence_permille
            );
        }
    }
}
