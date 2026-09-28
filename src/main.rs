#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use panic_probe as _;
use defmt_rtt as _;

#[embassy_executor::main(executor = "embassy_executor::Executor", entry = "cortex_m_rt::entry")]
async fn main(_spawner: Spawner) {
  let p = embassy_rp::init(Default::default());

  let mut led = Output::new(p.PIN_25, Level::Low);

  loop {
    info!("HIGH");
    led.set_high();
    Timer::after(Duration::from_millis(1000)).await;

    info!("LOW");
    led.set_low();
    Timer::after(Duration::from_millis(1000)).await;
  }
}
