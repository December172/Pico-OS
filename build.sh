#!/bin/bash

build(){
    profile="release"
    if [ "$1" == "dev" ]; then
        profile="debug"
    fi
    if [ ! -d "output" ]; then
        mkdir output
    fi
    cargo build --profile $1
    rm -rf output/*
    mv target/thumbv6m-none-eabi/$profile/kernel output/kernel.elf
    ./tools/picotool uf2 convert output/kernel.elf output/kernel.uf2
}

clean(){
    cargo clean
    cd output
    rm -rf *
}

case "$1" in 
    clean) clean ;;
    dev) build dev;;
    release) build release;;
    *) build dev;;
esac
