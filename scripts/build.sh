#!/usr/bin/env bash

cargo build --release
cp ./target/release/focusd ~/.cargo/bin/focusd
