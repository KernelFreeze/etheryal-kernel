[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/etheryal/etheryal-kernel/Tests)](https://github.com/etheryal/etheryal-kernel/actions)
[![Discord](https://img.shields.io/discord/805182661348818965)](https://discord.gg/dsY99BV2PT)
[![Lines of code](https://tokei.rs/b1/github/etheryal/etheryal-kernel?category=code)](https://github.com/XAMPPRocky/tokei)

# 😳 etheryal Kernel

**etheryal kernel** is an Open Source *capability-based* Kernel written in the Rust programming language. Unlike most historic kernels, *etheryal* components execute in the same address space (process), which contains software-isolated processes (SIPs). Each SIP has its own data and code layout, and is independent from other SIPs. These SIPs behave like normal processes, but avoid the cost of task-switches. *etheryal* uses a modular design based on Webassembly System Interface (WASI), containerizing drivers and user-space applications in a safe lightweight sandbox (WASM). Just like [Singularity](https://en.wikipedia.org/wiki/Singularity_(operating_system)), *etheryal* internal security uses type safety instead of hardware memory protection.

# ❤ Features

- Focused on performance and safety.
- Webassembly (WASM) runtime and Webassembly System Interface (WASI) implementation.
- Lightweight modular design.

# 🦀 License
*etheryal* is licensed under the permissive MIT license.

# ⚒ Building

You can build a Kernel binary with just `cargo make`.

```bash
cargo install --force cargo-make
cargo make build
```

# 🥳 Running

You can start a QEMU virtual machine running a booteable image generated with our tool `etheryal-bootimage` with `cargo make`.

```bash
cargo install --force cargo-make
cargo make run
```