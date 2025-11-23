![STM32F303RE Embsassy Blink Driver](https://github.com/mytechnotalent/stm32f303re_embassy_blink_driver/blob/main/STM32F303RE-Embassy-Blink-Driver.png?raw=true)

## FREE Reverse Engineering Self-Study Course [HERE](https://github.com/mytechnotalent/Reverse-Engineering-Tutorial)

<br>

# STM32F303RE Embassy Blink Driver

Embedded Rust w/ Embassy blink driver.

## Required Hardware

- **[STM32 Nucleo-F303RE Development Board](https://www.st.com/en/evaluation-tools/nucleo-f303re.html)**  
    - Development board based on the STM32F303RET6 microcontroller (ARM Cortex-M4F @ 72MHz).  
    - Features: Onboard ST-LINK/V2-1 debugger, Arduino Uno V3 connectivity, 512KB Flash, 64KB RAM.  
    - Ideal for embedded Rust development, async/await experimentation, and Embassy ecosystem learning.

- **USB Mini-B Cable**  
    - For powering the board and programming via ST-LINK.

## 🚀 Quick Start

```bash
# Connect your STM32 Nucleo board via USB
# Run the installation script to set up dependencies
./install.sh

# Build and flash
cargo run --release
```

That's it! The firmware will build and flash automatically.

## What's Included

- **GPIO Control**: Async LED blink on PA5 (green LED LD2)
- **UART Output**: Status messages via USART2 (ST-Link VCP at 115200 baud)
- **Embassy Async Runtime**: Clean async/await implementation for STM32F303RE

## Hardware Pin Mapping

- **LED (LD2)**: PA5 (GPIO output)
- **UART TX**: PA2 (USART2, connected to ST-Link VCP)
- **User Button**: PC13 (not used in this example)

## Building

This project uses Embassy from crates.io with STM32F303RE support.

```bash
# Install the ARM Cortex-M4F target (already done if you followed setup)
rustup target add thumbv7em-none-eabihf

# Build
cargo build --release
```

## Flashing to STM32F303RE

### Method 1: probe-run (Recommended)

The easiest method using the integrated ST-LINK debugger:

```bash
# Install probe-run if not already installed
cargo install probe-run

# Build and flash
cargo run --release
```

The `.cargo/config.toml` is configured to use `probe-run` as the runner.

### Method 2: probe-rs debug

For interactive debugging:

```bash
# Install probe-rs
cargo install probe-rs-tools --locked

# Build
cargo build --target thumbv7em-none-eabihf

# Start debug session
probe-rs debug --chip STM32F303RE --exe target/thumbv7em-none-eabihf/debug/stm32f303re-blink
```

### Method 3: VS Code Debug (OpenOCD)

If you have VS Code with the Cortex-Debug extension:

```bash
# Install OpenOCD
brew install openocd

# Install ARM GDB
brew tap ArmMbed/homebrew-formulae
brew install arm-none-eabi-gcc

# Press F5 in VS Code to build and debug
```

The project includes `.vscode/launch.json` configured for OpenOCD debugging.

### Method 4: st-flash

Using ST-Link utilities directly:

```bash
# Install stlink tools
brew install stlink

# Build
cargo build --release --target thumbv7em-none-eabihf

# Convert to binary
cargo objcopy --release --target thumbv7em-none-eabihf -- -O binary target/thumbv7em-none-eabihf/release/stm32f303re-blink firmware.bin

# Flash
st-flash write firmware.bin 0x8000000
```

## Dependencies Note

The `Cargo.toml` uses published Embassy crates from crates.io:
- `embassy-stm32 = "0.4.0"`
- `embassy-executor = "0.9.1"`
- `embassy-time = "0.5.0"`

## Key Features

1. **Async/Await**: LED blink runs asynchronously using Embassy timers
2. **Type Safety**: Rust's type system prevents many common embedded bugs
3. **No Blocking Loops**: Uses `embassy_time::Timer` for clean delay handling
4. **UART Logging**: Real-time status messages via ST-Link VCP

## Serial Output

Connect to the virtual COM port to see status messages:

```bash
# macOS
screen /dev/tty.usbmodem* 115200

# Linux
screen /dev/ttyACM0 115200
```

Output example:
```
LED ON
LED OFF
LED ON
LED OFF
```

## Generate API Documentation

```bash
cargo doc --no-deps --document-private-items --open
```

## Memory Layout

- **FLASH**: 512KB starting at 0x08000000
- **RAM**: 64KB starting at 0x20000000

Defined in `memory.x` for the STM32F303RET6.

---

## License

[MIT](https://github.com/mytechnotalent/stm32f303re_blink/blob/main/LICENSE)
