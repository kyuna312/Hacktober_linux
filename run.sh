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

# Run QEMU with optimized configuration for M1
qemu-system-aarch64 \
    -M virt,highmem=off \
    -cpu max \
    -m 128M \
    -device ramfb \
    -device virtio-gpu-pci,xres=1024,yres=768,max_outputs=1 \
    -device virtio-keyboard-pci \
    -device virtio-mouse-pci \
    -display default,show-cursor=on \
    -serial mon:stdio \
    -kernel target/aarch64-unknown-none/release/hacktober_linux \
    -append "console=ttyAMA0 keep_bootcon" \
    -d guest_errors

# Note: To exit, press Ctrl+C or close the QEMU window
