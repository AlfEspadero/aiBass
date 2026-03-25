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
const REG_STATUS: u8 = 0x1E;
const REG_OUTX_L_XL: u8 = 0x28;
const SAMPLE_PERIOD_MS: u64 = 2;
const EFFECTIVE_SAMPLE_RATE_HZ: u32 = 416;
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

    // Accelerometer: 416 Hz, +/-2g (LPF disabled for clearer vibration content).
    i2c.write(LSM6DSL_ADDR, &[REG_CTRL1_XL, 0x60]).await?;

    // Gyroscope: 416 Hz, 250 dps.
    i2c.write(LSM6DSL_ADDR, &[REG_CTRL2_G, 0x60]).await?;

    Ok(())
}

fn build_pitch_input(ring: &SampleRingBuffer) -> Option<PitchInput> {
    if ring.len < BUFFER_LEN {
        return None;
    }

    let mut values = [0i32; SAMPLE_WINDOW_LEN];
    let start = ring.write_idx;
    let mut mean_x: i64 = 0;
    let mut mean_y: i64 = 0;
    let mut mean_z: i64 = 0;
    let mut energy_x: i64 = 0;
    let mut energy_y: i64 = 0;
    let mut energy_z: i64 = 0;
    let mut x = [0i32; SAMPLE_WINDOW_LEN];
    let mut y = [0i32; SAMPLE_WINDOW_LEN];
    let mut z = [0i32; SAMPLE_WINDOW_LEN];

    for i in 0..SAMPLE_WINDOW_LEN {
        let idx = (start + i) % SAMPLE_WINDOW_LEN;
        x[i] = ring.data[idx].ax as i32;
        y[i] = ring.data[idx].ay as i32;
        z[i] = ring.data[idx].az as i32;
        mean_x += x[i] as i64;
        mean_y += y[i] as i64;
        mean_z += z[i] as i64;
    }

    mean_x /= SAMPLE_WINDOW_LEN as i64;
    mean_y /= SAMPLE_WINDOW_LEN as i64;
    mean_z /= SAMPLE_WINDOW_LEN as i64;

    for i in 0..SAMPLE_WINDOW_LEN {
        x[i] -= mean_x as i32;
        y[i] -= mean_y as i32;
        z[i] -= mean_z as i32;
        energy_x += (x[i] as i64) * (x[i] as i64);
        energy_y += (y[i] as i64) * (y[i] as i64);
        energy_z += (z[i] as i64) * (z[i] as i64);
    }

    if energy_x >= energy_y && energy_x >= energy_z {
        values.copy_from_slice(&x);
    } else if energy_y >= energy_x && energy_y >= energy_z {
        values.copy_from_slice(&y);
    } else {
        values.copy_from_slice(&z);
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
    let mut raw = [0u8; 6];
    let mut status = [0u8; 1];

    loop {
        // Poll status register; bit0 indicates new accel data available.
        if let Err(e) = i2c.write_read(LSM6DSL_ADDR, &[REG_STATUS], &mut status).await {
            warn!("IMU status read failed: {:?}", e);
            Timer::after(Duration::from_millis(SAMPLE_PERIOD_MS)).await;
            continue;
        }

        if status[0] & 0x01 == 0 {
            Timer::after(Duration::from_millis(SAMPLE_PERIOD_MS)).await;
            continue;
        }

        if let Err(e) = i2c.write_read(LSM6DSL_ADDR, &[REG_OUTX_L_XL], &mut raw).await {
            warn!("IMU sample read failed: {:?}", e);
            Timer::after(Duration::from_millis(SAMPLE_PERIOD_MS)).await;
            continue;
        }

        let sample = ImuSample {
            timestamp_ms: Instant::now().as_millis(),
            gx: 0,
            gy: 0,
            gz: 0,
            ax: i16::from_le_bytes([raw[0], raw[1]]),
            ay: i16::from_le_bytes([raw[2], raw[3]]),
            az: i16::from_le_bytes([raw[4], raw[5]]),
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
