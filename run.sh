#!/bin/bash

# Clean and build
cargo clean
cargo build --release

qemu-system-aarch64 \
    -machine virt,accel=hvf \
    -cpu cortex-a72 \
    -m 512M \
    -device virtio-gpu-pci,xres=1024,yres=768 \
    -device virtio-keyboard-pci \
    -device virtio-mouse-pci \
    -display cocoa,show-cursor=on \
    -kernel target/aarch64-unknown-none/release/nyannix
