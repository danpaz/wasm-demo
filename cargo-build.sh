#!/bin/sh

installpkg() {
    rustup update
    rustup target add wasm32-unknown-unknown
    cargo install --git https://github.com/alexcrichton/wasm-gc
}

# if you need update & install packages
installpkg

# build
cargo build --release --target wasm32-unknown-unknown
wasm-gc ./target/wasm32-unknown-unknown/release/wasm_demo.wasm

# the result
ls -lh target/wasm32-unknown-unknown/release/*.wasm
