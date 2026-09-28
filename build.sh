#!/bin/bash

build(){
    board="${2:-pico}"
    profile="release"
    if [ "$1" == "dev" ]; then
        profile="debug"
    fi
    if [ ! -d "output" ]; then
        mkdir output
    fi

    case "$board" in
        pico2|rp2350)
            target="thumbv8m.main-none-eabi"
            cargo build --profile "$1" --target "$target" -Zbuild-std=core \
                --no-default-features --features Pico2
            ;;
        *)
            target="thumbv6m-none-eabi"
            cargo build --profile "$1" -Zjson-target-spec
            ;;
    esac

    rm -rf output/*
    mv "target/$target/$profile/kernel" output/kernel.elf
    ./tools/picotool uf2 convert output/kernel.elf output/kernel.uf2
    arm-none-eabi-objdump -d -Mforce-thumb output/kernel.elf > output/disassembly.S
}

clean(){
    cargo clean
    cd output
    rm -rf *
}

case "$1" in 
    clean) clean ;;
    dev) build dev "$2";;
    release) build release "$2";;
    *) build dev "$2";;
esac
