# Learning-Rust
This repo contains the notes I took while learning the Rust programming language, the sample Rust programs I wrote, and the exercises. Don't hesitate to contribute and fork the repo.


# First Steps
## installation
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
rustup update
rustc --version

### creating project with cargo
cargo new hello_cargo
cd hello_cargo

### compile and run cargo project 
cargo build
./target/debug/hello_cargo