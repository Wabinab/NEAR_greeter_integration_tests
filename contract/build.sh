#!/bin/bash
set -e

export WASM=greeter.wasm

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/$WASM res/