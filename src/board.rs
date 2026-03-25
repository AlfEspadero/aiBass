use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::i2c::{self, I2c, Master};
use embassy_stm32::mode::Async;
use embassy_stm32::{bind_interrupts, dma};

bind_interrupts!(struct Irqs {
	I2C2_EV => i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C2>;
	I2C2_ER => i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C2>;
	DMA1_CHANNEL4 => dma::InterruptHandler<embassy_stm32::peripherals::DMA1_CH4>;
	DMA1_CHANNEL5 => dma::InterruptHandler<embassy_stm32::peripherals::DMA1_CH5>;
});

pub type ImuI2c = I2c<'static, Async, Master>;

pub struct Board {
	pub led: Output<'static>,
	#[allow(dead_code)]
	pub imu_i2c: ImuI2c,
}

impl Board {
	pub fn init() -> Self {
		let p = embassy_stm32::init(Default::default());
		let led = Output::new(p.PB14, Level::High, Speed::Low);

		// LSM6DSL on B-L4S5I-IOT01A is wired to I2C2 (PB10/PB11) in ST examples.
		let imu_i2c = I2c::new(
			p.I2C2,
			p.PB10,
			p.PB11,
			p.DMA1_CH4,
			p.DMA1_CH5,
			Irqs,
			Default::default(),
		);

		Self { led, imu_i2c }
	}
}
