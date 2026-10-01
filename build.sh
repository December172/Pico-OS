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

    # The cargo aliases in .cargo/config.toml select the target triple, the
    # architecture-specific core and the feature set for each board.
    case "$board" in
        pico2|rp2350)
            alias="pico2350"
            target="thumbv8m.main-none-eabi"
            # RP2350-E10 (A2): absolute block so UF2 drag-and-drop works when a
            # partition table is present. Ignored by A3+ bootroms.
            absBlock="--abs-block"
            ;;
        *)
            alias="pico2040"
            target="thumbv6m-none-eabi"
            absBlock=""
            ;;
    esac

    cargo "$alias" --profile "$1"

    rm -rf output/*
    mv "target/$target/$profile/kernel" output/kernel.elf
    ./tools/picotool uf2 convert output/kernel.elf output/kernel.uf2 $absBlock
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
