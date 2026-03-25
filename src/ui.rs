use defmt::info;
use embassy_executor::task;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Receiver;
use embassy_time::{Duration, Timer};

pub type UiStateReceiver = Receiver<'static, ThreadModeRawMutex, UiState, 4>;

#[derive(Copy, Clone)]
pub enum TuningDirection {
	Flat,
	InTune,
	Sharp,
	NoPitch,
}

#[derive(Copy, Clone)]
pub struct UiState {
	pub note: &'static str,
	pub freq_hz_x100: u32,
	pub cents_x10: i32,
	pub confidence_permille: u16,
	pub direction: TuningDirection,
}

impl UiState {
	pub fn no_pitch() -> Self {
		Self {
			note: "-",
			freq_hz_x100: 0,
			cents_x10: 0,
			confidence_permille: 0,
			direction: TuningDirection::NoPitch,
		}
	}

	pub fn from_pitch(
		note: &'static str,
		freq_hz_x100: u32,
		cents_x10: i32,
		confidence_permille: u16,
	) -> Self {
		let direction = if cents_x10 < -50 {
			TuningDirection::Flat
		} else if cents_x10 > 50 {
			TuningDirection::Sharp
		} else {
			TuningDirection::InTune
		};

		Self {
			note,
			freq_hz_x100,
			cents_x10,
			confidence_permille,
			direction,
		}
	}
}

#[task]
pub async fn ui_task(rx: UiStateReceiver) {
	loop {
		let state = rx.receive().await;
		match state.direction {
			TuningDirection::NoPitch => info!("ui: no stable pitch"),
			TuningDirection::Flat => info!(
				"ui: note={} {}.{:02}Hz flat {}.{}c conf={}",
				state.note,
				state.freq_hz_x100 / 100,
				state.freq_hz_x100 % 100,
				state.cents_x10 / 10,
				(state.cents_x10.abs() % 10),
				state.confidence_permille
			),
			TuningDirection::InTune => info!(
				"ui: note={} {}.{:02}Hz in-tune {}.{}c conf={}",
				state.note,
				state.freq_hz_x100 / 100,
				state.freq_hz_x100 % 100,
				state.cents_x10 / 10,
				(state.cents_x10.abs() % 10),
				state.confidence_permille
			),
			TuningDirection::Sharp => info!(
				"ui: note={} {}.{:02}Hz sharp +{}.{}c conf={}",
				state.note,
				state.freq_hz_x100 / 100,
				state.freq_hz_x100 % 100,
				state.cents_x10 / 10,
				(state.cents_x10.abs() % 10),
				state.confidence_permille
			),
		}

		// Output deadband period for serial readability in MVP mode.
		Timer::after(Duration::from_millis(250)).await;
	}
}
