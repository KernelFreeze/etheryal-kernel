# Building
```bash
cargo install --force cargo-make

git clone https://github.com/etheryal/kernel
git clone https://github.com/rust-osdev/bootloader --branch uefi
cd bootloader

cargo builder --kernel-binary ../kernel/target/x86_64/debug/kernel --kernel-manifest ../kernel/Cargo.toml
```