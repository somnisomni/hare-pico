use embassy_rp::dma::{InterruptHandler as DmaInterruptHandler};
use embassy_rp::{Peri, bind_interrupts};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIN_0, PIN_1, UART0};
use embassy_rp::uart::{Async, Config, Uart, UartRx, UartTx, InterruptHandler as UartInterruptHandler};

bind_interrupts!(struct Irqs {
  UART0_IRQ => UartInterruptHandler<UART0>;
  DMA_IRQ_0 => DmaInterruptHandler<DMA_CH0>, DmaInterruptHandler<DMA_CH1>;
});

pub fn create_uart(
    uart: Peri<'static, UART0>,
    tx_pin: Peri<'static, PIN_0>,
    rx_pin: Peri<'static, PIN_1>,
    dma_ch0: Peri<'static, DMA_CH0>,
    dma_ch1: Peri<'static, DMA_CH1>,
    baudrate: Option<u32>)
  -> (UartTx<'static, Async>, UartRx<'static, Async>) {
  let mut cfg = Config::default();
  cfg.baudrate = baudrate.unwrap_or(115_200);

  let uart = Uart::new(uart, tx_pin, rx_pin, Irqs, dma_ch0, dma_ch1, cfg);

  let (tx, rx) = uart.split();
  (tx, rx)
}
