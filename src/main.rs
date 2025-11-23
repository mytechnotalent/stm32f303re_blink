//! STM32F303RE Nucleo LED Blink Example
//!
//! This application demonstrates basic embedded Rust development using the Embassy framework.
//! It blinks the onboard LED (LD2 on PA5) at a configurable interval while sending status
//! messages over UART2 to the ST-Link virtual COM port.
//!
//! # Hardware
//! - Board: STM32F303RE Nucleo
//! - MCU: STM32F303RET6 (ARM Cortex-M4F @ 72MHz)
//! - LED: Green LED (LD2) on PA5
//! - UART: USART2 on PA2 (TX) via ST-Link VCP
//!
//! # Features
//! - Async/await with Embassy executor
//! - Real-time debug logging via RTT (defmt)
//! - UART serial output at 115200 baud
//! - Low power and optimized for embedded systems
//!
//! # Usage
//! Flash to board: `cargo run --release`
//! Monitor serial: `screen /dev/tty.usbmodem* 115200`

#![no_std]
#![no_main]

mod config;

use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

/// Main application entry point
///
/// Initializes the STM32 peripherals and runs an infinite loop that:
/// 1. Turns the LED on and sends "LED ON" via UART
/// 2. Waits for the configured interval
/// 3. Turns the LED off and sends "LED OFF" via UART
/// 4. Waits for the configured interval
/// 5. Repeats
///
/// # Arguments
/// * `_spawner` - Embassy task spawner (unused in this simple example)
///
/// # Panics
/// Never returns. Runs indefinitely until power loss or reset.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize STM32 peripherals with default configuration
    let p = embassy_stm32::init(Default::default());

    // Initialize hardware (LED and UART)
    let mut hw = config::Hardware::init(p);

    // Main application loop - blink LED and send UART messages
    loop {
        // Turn LED on and notify via UART
        let _ = hw.usart.blocking_write(config::messages::LED_ON);
        hw.led.set_high();
        Timer::after_millis(config::LED_BLINK_INTERVAL_MS).await;

        // Turn LED off and notify via UART
        let _ = hw.usart.blocking_write(config::messages::LED_OFF);
        hw.led.set_low();
        Timer::after_millis(config::LED_BLINK_INTERVAL_MS).await;
    }
}
