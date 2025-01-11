#!/bin/bash

# Set environment variables
export PATH="/opt/homebrew/bin:$PATH"
export CC_aarch64_unknown_none=/opt/homebrew/bin/aarch64-none-elf-gcc
export AR_aarch64_unknown_none=/opt/homebrew/bin/aarch64-none-elf-ar

# Kill any existing QEMU processes
pkill qemu-system-aarch64 2>/dev/null || true

# Clean and build
cargo clean
cargo build --release

# Run QEMU with fixed display configuration for macOS
qemu-system-aarch64 \
    -M virt \
    -cpu cortex-a72 \
    -m 128M \
    -device ramfb \
    -device virtio-gpu-pci,xres=800,yres=600,max_outputs=1 \
    -device virtio-keyboard-pci \
    -device virtio-mouse-pci \
    -display cocoa \
    -serial stdio \
    -kernel target/aarch64-unknown-none/release/nyannix \
    -append "console=ttyAMA0" \
    -smp 1

# Note: To exit, press Ctrl+C or close the QEMU window
