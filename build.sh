#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$SCRIPT_DIR/.bash.linux.rc"

rm -rf "$SCRIPT_DIR/jniLibs"
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t  x86_64 -o "$SCRIPT_DIR/jniLibs" build --release

#
cp -Rf "$SCRIPT_DIR/jniLibs" "$SCRIPT_DIR/android/app/src/main"