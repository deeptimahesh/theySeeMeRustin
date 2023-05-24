# Rust: An Overview

## Important Commands
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Check if Rust is installed
rustc --version

# Cargo Commands
cargo new hello_world
cd hello_world

cargo build
cargo run
# OR
rustc src/main.rs
./main

# Release
cargo build --release
```