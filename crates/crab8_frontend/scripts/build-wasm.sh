#!/bin/bash

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
PWD=$(pwd)
CRATE_NAME=crab8_frontend
BUILD_MODE=release
TARGET_DIR=../../target
OUT_DIR=./wasm/built/
ARCH=wasm32-unknown-unknown

cd $SCRIPT_DIR/..

echo "Building..."
cargo build -p $CRATE_NAME --release --target $ARCH

echo "Generating bindings..."
wasm-bindgen --out-dir $OUT_DIR --target web $TARGET_DIR/$ARCH/$BUILD_MODE/$CRATE_NAME.wasm

cd $PWD
