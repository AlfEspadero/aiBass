#![no_std]
#![no_main]

mod acquisition;
mod board;
mod pitch;
mod ui;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

static PITCH_INPUT_CH: Channel<ThreadModeRawMutex, pitch::PitchInput, 4> = Channel::new();
static UI_STATE_CH: Channel<ThreadModeRawMutex, ui::UiState, 4> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let mut board = board::Board::init();
	info!("aiBass firmware boot");
	pitch::run_validation_harness();

	spawner.spawn(unwrap!(acquisition::acquisition_task(
		board.imu_i2c,
		PITCH_INPUT_CH.sender()
	)));
	spawner.spawn(unwrap!(pitch::pitch_task(
		PITCH_INPUT_CH.receiver(),
		UI_STATE_CH.sender()
	)));
	spawner.spawn(unwrap!(ui::ui_task(UI_STATE_CH.receiver())));

	loop {
		board.led.set_high();
		Timer::after(Duration::from_millis(250)).await;
		board.led.set_low();
		Timer::after(Duration::from_millis(250)).await;
	}
}
