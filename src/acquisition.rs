use defmt::{info, warn};
use embassy_executor::task;
use embassy_sync::channel::Sender;
use embassy_time::{Duration, Instant, Timer};

use crate::board::ImuI2c;
use crate::pitch::{PitchInput, SAMPLE_WINDOW_LEN};

const LSM6DSL_ADDR: u8 = 0x6A;
const REG_WHO_AM_I: u8 = 0x0F;
const REG_CTRL3_C: u8 = 0x12;
const REG_CTRL1_XL: u8 = 0x10;
const REG_CTRL2_G: u8 = 0x11;
const REG_OUTX_L_G: u8 = 0x22;
const SAMPLE_PERIOD_MS: u64 = 4;
const EFFECTIVE_SAMPLE_RATE_HZ: u32 = 208;
const BUFFER_LEN: usize = SAMPLE_WINDOW_LEN;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ImuSample {
    pub timestamp_ms: u64,
    pub gx: i16,
    pub gy: i16,
    pub gz: i16,
    pub ax: i16,
    pub ay: i16,
    pub az: i16,
}

impl ImuSample {
    const fn zero() -> Self {
        Self {
            timestamp_ms: 0,
            gx: 0,
            gy: 0,
            gz: 0,
            ax: 0,
            ay: 0,
            az: 0,
        }
    }
}

struct SampleRingBuffer {
    data: [ImuSample; BUFFER_LEN],
    write_idx: usize,
    len: usize,
}

impl SampleRingBuffer {
    const fn new() -> Self {
        Self {
            data: [ImuSample::zero(); BUFFER_LEN],
            write_idx: 0,
            len: 0,
        }
    }

    fn push(&mut self, sample: ImuSample) {
        self.data[self.write_idx] = sample;
        self.write_idx = (self.write_idx + 1) % BUFFER_LEN;
        if self.len < BUFFER_LEN {
            self.len += 1;
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

async fn configure_imu(i2c: &mut ImuI2c) -> Result<(), embassy_stm32::i2c::Error> {
    // Enable register auto-increment (IF_INC=1) for burst reads.
    i2c.write(LSM6DSL_ADDR, &[REG_CTRL3_C, 0x04]).await?;

    // Accelerometer: 208 Hz, +/-2g.
    i2c.write(LSM6DSL_ADDR, &[REG_CTRL1_XL, 0x50]).await?;

    // Gyroscope: 208 Hz, 250 dps.
    i2c.write(LSM6DSL_ADDR, &[REG_CTRL2_G, 0x50]).await?;

    Ok(())
}

fn build_pitch_input(ring: &SampleRingBuffer) -> Option<PitchInput> {
    if ring.len < BUFFER_LEN {
        return None;
    }

    let mut values = [0i32; SAMPLE_WINDOW_LEN];
    let start = ring.write_idx;
    let mut mean: i64 = 0;

    for i in 0..SAMPLE_WINDOW_LEN {
        let idx = (start + i) % SAMPLE_WINDOW_LEN;
        // Use a signed single-axis vibration signal to preserve fundamental frequency.
        let v = ring.data[idx].ax as i32;
        values[i] = v;
        mean += v as i64;
    }

    mean /= SAMPLE_WINDOW_LEN as i64;
    for v in &mut values {
        *v -= mean as i32;
    }

    Some(PitchInput {
        values,
        sample_rate_hz: EFFECTIVE_SAMPLE_RATE_HZ,
    })
}

#[task]
pub async fn acquisition_task(mut i2c: ImuI2c, pitch_tx: Sender<'static, embassy_sync::blocking_mutex::raw::ThreadModeRawMutex, PitchInput, 4>) {
    let mut whoami = [0u8; 1];
    match i2c.write_read(LSM6DSL_ADDR, &[REG_WHO_AM_I], &mut whoami).await {
        Ok(()) => info!("LSM6DSL WHO_AM_I=0x{:02x}", whoami[0]),
        Err(e) => {
            warn!("IMU WHO_AM_I read failed: {:?}", e);
            return;
        }
    }

    if let Err(e) = configure_imu(&mut i2c).await {
        warn!("IMU configuration failed: {:?}", e);
        return;
    }

    let mut ring = SampleRingBuffer::new();
    let mut raw = [0u8; 12];

    loop {
        if let Err(e) = i2c
            .write_read(LSM6DSL_ADDR, &[REG_OUTX_L_G | 0x80], &mut raw)
            .await
        {
            warn!("IMU sample read failed: {:?}", e);
            Timer::after(Duration::from_millis(SAMPLE_PERIOD_MS)).await;
            continue;
        }

        let sample = ImuSample {
            timestamp_ms: Instant::now().as_millis(),
            gx: i16::from_le_bytes([raw[0], raw[1]]),
            gy: i16::from_le_bytes([raw[2], raw[3]]),
            gz: i16::from_le_bytes([raw[4], raw[5]]),
            ax: i16::from_le_bytes([raw[6], raw[7]]),
            ay: i16::from_le_bytes([raw[8], raw[9]]),
            az: i16::from_le_bytes([raw[10], raw[11]]),
        };

        ring.push(sample);

        if let Some(input) = build_pitch_input(&ring) {
            if let Err(_e) = pitch_tx.try_send(input) {
                // Keep sampling; dropping occasional frames is preferable to blocking acquisition.
            }
        }

        if ring.len() % 32 == 0 {
            info!(
                "acq ring fill={} fs={}Hz last_ax={} last_ay={} last_az={}",
                ring.len(),
                EFFECTIVE_SAMPLE_RATE_HZ,
                sample.ax,
                sample.ay,
                sample.az
            );
        }

        Timer::after(Duration::from_millis(SAMPLE_PERIOD_MS)).await;
    }
}
