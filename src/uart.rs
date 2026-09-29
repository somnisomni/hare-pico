use defmt::{error, info};
use embassy_rp::uart::{Async, UartRx, UartTx};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use heapless::{String};

const UART_BUFFER_SIZE: usize = 64;

pub static UART_TX_CHANNEL: Channel<CriticalSectionRawMutex, String<UART_BUFFER_SIZE>, 4> = Channel::new();

#[embassy_executor::task]
pub async fn uart_tx_task(mut tx: UartTx<'static, Async>) {
  loop {
    let message = UART_TX_CHANNEL.receive().await;

    if let Err(e) = tx.write(message.as_bytes()).await {
      error!("UART TX error: {:?}", e);
    }
  }
}

#[embassy_executor::task]
pub async fn uart_rx_task(mut rx: UartRx<'static, Async>) {
  let mut char_buffer = [0u8; 1];
  let mut line_buffer: String<UART_BUFFER_SIZE> = String::new();

  loop {
    match rx.read(&mut char_buffer).await {
      Ok(_) => {
        let byte = char_buffer[0];

        if byte != b'\n' && byte != b'\r' {
          if line_buffer.push(byte as char).is_err() {
            error!("Line buffer overflow, clearing buffer");
            line_buffer.clear();
          }

          continue;
        }

        if !line_buffer.is_empty() {
          if let Ok(message) = core::str::from_utf8(line_buffer.as_bytes()) {
            info!("Line received: {}", message);

            // TODO
            uart_send_str(message).await; // TEMP
          } else {
            error!("Received non-UTF8 data");
          }

          line_buffer.clear();
        }
      }

      Err(e) => {
        error!("UART RX error: {:?}", e);
        Timer::after(Duration::from_millis(50)).await;
      }
    }
  }
}

#[inline]
pub async fn uart_send(message: String<UART_BUFFER_SIZE>) {
  UART_TX_CHANNEL.send(message).await;
}

pub async fn uart_send_str(message: &str) {
  let mut str_buffer: String<UART_BUFFER_SIZE> = String::new();

  match str_buffer.push_str(message) {
    Ok(_) => uart_send(str_buffer).await,
    Err(_) => error!("Failed to create string buffer while sending message over UART: {:?}", message),
  }
}

pub async fn uart_send_bytes(message: &[u8]) {
  let message_str = core::str::from_utf8(message);

  match message_str {
    Ok(msg) => uart_send_str(msg).await,
    Err(_) => error!("Failed to convert bytes to UTF-8 string while sending message over UART: {:?}", message),
  }
}
