#!/bin/bash

echo "1. Building.."
cargo build

echo "2. Purging Assets..."
rm -rf "$PWD/target/debug/assets"

echo "3. Copying Assets..."
cp -r "$PWD/assets" "$PWD/target/debug/assets"

echo "4. Running Executable..."
cargo run
