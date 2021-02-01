![GitHub Workflow Status](https://img.shields.io/github/workflow/status/etheryal/etheryal-kernel/Build)

# 😳 etheryal Kernel

etheryal Kernel is an Open Source capability-based Kernel written in Rust programming language. Uses a new design based on Webassembly System Interface, containerizing drivers and user-space applications in a safe sandbox.

# ❤ Features

- Kernel-mode WASM runtime/sandbox.
- Very lightweight modular design.
- Webassembly System Interface implementation.

# ⚒ Building

You can build a Kernel image with just `cargo make`, but is often desireable to create a booteable image for debugging purposes. Our `bootimage` tool creates a UEFI and BIOS booteable `.img` files, that you can run in your favorite emulator or on bare-metal hardware.

```bash
cargo install --git https://github.com/etheryal/bootimage --branch main
cargo install --force cargo-make

cargo make build-image
```