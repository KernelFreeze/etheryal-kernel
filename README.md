[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/etheryal/etheryal-kernel/Build)](https://github.com/etheryal/etheryal-kernel/actions)
[![Discord](https://img.shields.io/discord/805182661348818965)](https://discord.gg/dsY99BV2PT)
[![Lines of code](https://tokei.rs/b1/github/etheryal/etheryal-kernel?category=code)](https://github.com/XAMPPRocky/tokei)

# 😳 etheryal Kernel

**etheryal microkernel** is an Open Source *capability-based* Kernel written in the Rust programming language. *etheryal Kernel* uses a modular design based on Webassembly System Interface (WASI), containerizing drivers and user-space applications in a safe lightweight sandbox (WASM), without requiring slow kernel-mode to user-mode switches on every system call and interrupt, that may happen really often in common workloads. Just like [Singularity](https://en.wikipedia.org/wiki/Singularity_(operating_system)), etheryal internal security uses type safety instead of hardware memory protection.

Unlike most historic microkernels, etheryal components execute in the same address space (process), which contains software-isolated processes (SIPs). Each SIP has its own data and code layout, and is independent from other SIPs. These SIPs behave like normal processes, but avoid the cost of task-switches.

# ❤ Features

- Focused on performance and safety.
- Kernel-mode Webassembly runtime/sandbox.
- Lightweight modular design.
- Webassembly System Interface implementation.

# 🦀 License
etheryal is licensed under the permissive MIT license.

# ⚒ Building

You can build a Kernel binary with just `cargo make`.

```bash
cargo install --force cargo-make
cargo make build
```

# 🥳 Running

You can start a QEMU virtual machine running our `bootimage` with `cargo make`.

```bash
cargo install --force cargo-make
cargo make run
```