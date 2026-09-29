#![no_std]
#![no_main]

mod peripherals;
mod uart;

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use defmt_rtt as _;
use panic_reset as _;

#[embassy_executor::main(executor = "embassy_executor::Executor", entry = "cortex_m_rt::entry")]
async fn main(spawner: Spawner) {
  let p = embassy_rp::init(Default::default());

  let mut led = Output::new(p.PIN_25, Level::Low);

  let (uart_tx, uart_rx) = peripherals::create_uart(p.UART0, p.PIN_0, p.PIN_1, p.DMA_CH0, p.DMA_CH1, None);

  spawner.spawn(uart::uart_tx_task(uart_tx).unwrap());
  spawner.spawn(uart::uart_rx_task(uart_rx).unwrap());

  loop {
    led.set_high();
    uart::uart_send_bytes(b"LED HIGH\r\n").await;
    Timer::after(Duration::from_millis(1000)).await;

    led.set_low();
    uart::uart_send_bytes(b"LED LOW\r\n").await;
    Timer::after(Duration::from_millis(1000)).await;
  }
}
