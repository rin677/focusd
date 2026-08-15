#!/usr/bin/env bash

cargo build --release
rm ~/.cargo/bin/focusd
cp ./target/release/focusd ~/.cargo/bin/focusd
