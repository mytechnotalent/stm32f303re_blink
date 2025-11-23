#!/bin/bash
set -e

# Install dependencies for STM32F303RE Embassy Rust development
echo "Installing dependencies for STM32F303RE Embassy Rust development..."
echo ""

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Please install it first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Confirm Rust installation
echo "✓ Rust is installed"

# Install ARM Cortex-M4F target
echo "Installing thumbv7em-none-eabihf target..."
rustup target add thumbv7em-none-eabihf

# Install probe-rs for flashing
if command -v probe-rs &> /dev/null; then
    echo "✓ probe-rs is already installed"
else
    echo "Installing probe-rs..."
    cargo install probe-rs-tools --locked
    echo "✓ probe-rs installed successfully"
fi

# give final instructions to run the project
echo "Command to run the project:"
echo "cargo run"
echo "cargo run --release"
