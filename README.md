# Building
```bash
rustup component add llvm-tools-preview
rustup component add rust-src

git clone kernel
git clone https://github.com/rust-osdev/bootloader --branch uefi
cd bootloader

cargo builder --kernel-binary ../kernel/target/x86_64/debug/kernel --kernel-manifest ../kernel/Cargo.toml
```