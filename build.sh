#!/bin/bash

build(){
    cargo build --release
    mv target/thumbv6m-none-eabi/release/Pico-OS output/Pico-OS.elf
    ./tools/picotool uf2 convert output/Pico-OS.elf output/Pico-OS.uf2
}

clean(){
    cargo clean
    cd output
    rm -rf *
}

case "$1" in 
    clean) clean ;;
    build) build ;;
    *) build ;;
esac
