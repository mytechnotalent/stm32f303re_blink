//! Hardware configuration and initialization for STM32F303RE Nucleo board
//!
//! This module encapsulates all hardware-specific configuration including:
//! - Pin definitions and peripheral mappings
//! - Timing constants
//! - UART message definitions
//! - Hardware initialization routines
//!
//! # Design Philosophy
//! Configuration is centralized here to separate hardware concerns from application
//! logic, making the codebase more maintainable and portable.

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::UartTx;
use embassy_stm32::{mode::Blocking, Peripherals};

/// Small trait to abstract a blocking-write-capable UART transmitter.
///
/// We define a local trait and implement it for the concrete `UartTx` type
/// so the public API avoids exposing newer/unstable generic parameters.
pub trait WriteBlocking {
    fn blocking_write(&mut self, bytes: &[u8]) -> Result<(), ()>;
}

impl WriteBlocking for UartTx<'static, Blocking> {
    fn blocking_write(&mut self, bytes: &[u8]) -> Result<(), ()> {
        // Use the inherent blocking_write method provided by the blocking UartTx
        self.blocking_write(bytes).map_err(|_| ())
    }
}

/// LED blink interval in milliseconds
///
/// Controls the on/off period for the LED blink cycle.
/// Default: 500ms (resulting in 1Hz blink rate)
pub const LED_BLINK_INTERVAL_MS: u64 = 500;

/// UART serial message definitions
///
/// Pre-formatted messages sent over UART2 to the ST-Link virtual COM port.
/// Messages use CR+LF (\r\n) line endings for proper terminal display.
pub mod messages {
    /// LED state: ON - Sent when LED is activated
    #[allow(dead_code)]
    pub const LED_ON: &[u8] = b"LED ON\r\n";

    /// LED state: OFF - Sent when LED is deactivated
    #[allow(dead_code)]
    pub const LED_OFF: &[u8] = b"LED OFF\r\n";
}

/// Hardware abstraction containing all initialized peripherals
///
/// This structure owns the GPIO and UART peripherals after initialization,
/// providing a clean interface for the main application logic.
///
/// # Lifetimes
/// Uses 'static lifetime as peripherals are owned for the program duration.
///
/// # Fields
/// * `led` - GPIO output for the onboard LED (PA5)
/// * `usart` - UART transmitter for serial communication (USART2)
///
/// Returns an initialized LED `Output` and a blocking-write capable UART transmitter.
pub fn init(p: Peripherals) -> (Output<'static>, impl WriteBlocking) {
    // Initialize UART2 TX (connected to ST-Link VCP on PA2)
    // Configuration: 115200 baud, 8 data bits, no parity, 1 stop bit (8N1)
    let uart_config = embassy_stm32::usart::Config::default();
    // Create a blocking UART transmitter (no DMA) using the convenience constructor
    let usart = UartTx::new_blocking(p.USART2, p.PA2, uart_config).unwrap();

    // Configure PA5 as push-pull output for the onboard LED (LD2)
    // Initial state: LOW (LED off), Speed: Low (2MHz slew rate)
    let led = Output::new(p.PA5, Level::Low, Speed::Low);

    // Return initialized peripherals
    (led, usart)
}
