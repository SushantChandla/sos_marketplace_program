#!/bin/sh
cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist/program
solana program deploy /Users/sushantchandla/rust/shadow_of_strom_programs/marketplace/dist/program/marketplace.so