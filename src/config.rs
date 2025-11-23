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
use embassy_stm32::Peripherals;

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
pub struct Hardware {
    /// Onboard LED (LD2) - Green LED on PA5
    pub led: Output<'static, embassy_stm32::peripherals::PA5>,

    /// UART2 transmitter connected to ST-Link virtual COM port
    #[allow(dead_code)]
    pub usart: UartTx<'static, embassy_stm32::peripherals::USART2, embassy_stm32::dma::NoDma>,
}

impl Hardware {
    /// Initialize and configure all hardware peripherals
    ///
    /// Sets up:
    /// - USART2 on PA2 (TX) with default configuration (115200 baud, 8N1)
    /// - GPIO PA5 as push-pull output for LED control (initially LOW)
    ///
    /// # Arguments
    /// * `p` - STM32 peripheral singleton from embassy_stm32
    ///
    /// # Returns
    /// Initialized `Hardware` struct with configured peripherals
    ///
    /// # Panics
    /// Panics if UART initialization fails (unwrap on UART creation)
    ///
    /// # Hardware Details
    /// - USART2 is connected to the ST-Link virtual COM port on Nucleo boards
    /// - PA5 drives the green user LED (LD2) on the Nucleo-F303RE
    /// - No DMA is used for UART (polling mode via blocking_write)
    pub fn init(p: Peripherals) -> Self {
        // Initialize UART2 TX (connected to ST-Link VCP on PA2)
        // Configuration: 115200 baud, 8 data bits, no parity, 1 stop bit (8N1)
        let uart_config = embassy_stm32::usart::Config::default();
        let usart = UartTx::new(p.USART2, p.PA2, embassy_stm32::dma::NoDma, uart_config).unwrap();

        // Configure PA5 as push-pull output for the onboard LED (LD2)
        // Initial state: LOW (LED off), Speed: Low (2MHz slew rate)
        let led = Output::new(p.PA5, Level::Low, Speed::Low);

        // Return the initialized hardware struct
        Self { led, usart }
    }
}
