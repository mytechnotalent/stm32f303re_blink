# stm32f303re-blink

A minimal, production-minded example to blink an LED on an STM32F303RE using the Embassy async HAL and Cortex-M runtime.

**Project**
- **Name:** `stm32f303re-blink` (crate version `0.1.0`)
- **Purpose:** Demonstrate a simple asynchronous LED blink running on an STM32F303RE-based board using the Embassy ecosystem.

**Features**
- Small, dependency-focused example using `embassy-stm32` and `embassy-executor`.
- Configured for `thumbv7em-none-eabihf` targets.
- Release-friendly compiler flags (LTO, sensible debug info) in `Cargo.toml`.

**Prerequisites**
- macOS with developer tools (`zsh` shell).
- Rust toolchain (stable) and the embedded target:

```bash
rustup default stable
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools-preview
```

- Recommended utilities (choose one):
  - `probe-run` (recommended for probe-rs based debugging/running)
  - `st-flash` or `openocd` (for raw flashing)

Install helpful cargo tools:

```bash
cargo install probe-run
cargo install cargo-binutils
```

If you plan to flash with ST-Link and `st-flash`, install via Homebrew:

```bash
brew install stlink
# optionally: brew tap ArmMbed/homebrew-formulae && brew install arm-none-eabi-gcc
```

**Build (recommended)**

Build a release binary for the microcontroller target:

```bash
cargo build --release --target thumbv7em-none-eabihf
```

Run the built ELF directly using `probe-run` (connect your debug probe first):

```bash
probe-run target/thumbv7em-none-eabihf/release/stm32f303re-blink
```

If you prefer to produce a raw binary for flashing with `st-flash`:

```bash
cargo objcopy --release --target thumbv7em-none-eabihf -- -O binary target/thumbv7em-none-eabihf/release/stm32f303re-blink firmware.bin
st-flash write firmware.bin 0x8000000
```

Adjust the flash address (`0x08000000` or `0x8000000` depending on the tool) to match your board requirements.

**Project Structure**
- `Cargo.toml` : crate metadata and dependency configuration (Embassy + runtime settings).
- `memory.x` : linker memory layout for the STM32F303RE flash/RAM.
- `build.rs` : build script (if present) for embedding resources or generating code.
- `src/main.rs` : application entry point (async executor + LED blink logic).
- `src/config.rs` : board-specific pin/peripheral configuration.

**Memory Layout & Notes**
- The repo includes `memory.x`. Ensure the flash origin and size match your board (use the correct `FLASH` base address and RAM size for the F303RE device).
- Release profile in `Cargo.toml` enables `lto = true` and `opt-level = "z"` for size/speed tradeoffs. Adjust as needed.

**Debugging**
- `probe-run` provides a convenient run-and-debug experience using probe-rs. For advanced debugging, use `gdb` via `probe-rs` or `openocd` with an ST-Link.
- If you need RTT logs, the project already depends on `defmt`/`defmt-rtt` — use `defmt-print`/`probe-run` to capture logs.

**Generate API docs (local)**

To build the crate documentation (including private items and your pro docstrings) and open it in your browser:

```bash
cargo doc --no-deps --document-private-items --open
```

Notes:
- `--no-deps` prevents building docs for all dependencies (useful for embedded deps that may not doc-build on the host).
- `--document-private-items` includes private items and internal docstrings so you can inspect implementation details.
- The generated files live under `target/<target-triple>/doc/stm32f303re_blink` (or `target/doc` for host builds).

**Contributing**
- Keep examples minimal and focused.
- Prefer reproducible tool versions (document changes to toolchain or `Cargo.toml`).
- Add unit/integration tests where applicable and document new steps in this `README.md`.

- Add a `.cargo/config.toml` with a `runner = "probe-run"` entry for `cargo run`, or
- Add CI steps to build and verify the example on push.
