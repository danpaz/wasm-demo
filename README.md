# wasm-demo

Demo of rust compiled to web assembly.

![demo](demo.gif)

## Setup

Based on https://www.hellorust.com/demos/add/index.html.

    rustup update
    rustup install nightly
    rustup target add wasm32-unknown-unknown --toolchain nightly
    rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib src/main.rs -o main.big.wasm
    cargo install --git https://github.com/alexcrichton/wasm-gc
    wasm-gc main.big.wasm public/main.wasm

## Run

    python -m SimpleHTTPServer 8000

Navigate to http://localhost:8000/public/index.html
