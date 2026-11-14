#!/bin/bash

# Resolve all project-relative paths independently of the caller's directory.
set -euo pipefail
tools_directory="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd -- "$tools_directory/.."

usage(){
    echo "Usage: bash tools/build.sh [dev|release|clean] [board]" >&2
}

# Build kernel elf only; User images link should be done elsewhere.
build(){
    local board="$2"
    local profile="release"
    if [ "$1" == "dev" ]; then
        profile="debug"
    fi

    # Only RP2040 is currently supported. Both build scripts validate the
    # configured chip against this target before compiling either image.
    local target="thumbv6m-none-eabi"
    local -a commands=(cargo python3 arm-none-eabi-objcopy arm-none-eabi-objdump)
    local command
    for command in "${commands[@]}"; do
        if ! command -v "$command" >/dev/null 2>&1; then
            echo "Required build tool not found: $command" >&2
            return 1
        fi
    done

    # Cargo also honors target-dir in config files and CARGO_TARGET_DIR.
    local target_directory="$(cargo metadata --format-version=1 --no-deps | python3 -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])')"

    PICO_BOARD="$board" cargo build --profile "$1"

    # Publish the final artifacts only after every conversion has succeeded.
    (
        # Stage under Cargo's target directory; publish firmware and matching
        # debugger symbols together, independently of the build-cache location.
        staging="$(mktemp -d "$target_directory/.pico-build-XXXXXX")"
        trap 'rm -rf "$staging"' EXIT

        mkdir -p "$staging/output/symbols"
        cp "$target_directory/$target/$profile/kernel" "$staging/output/kernel.elf"
        arm-none-eabi-objdump -d -Mforce-thumb "$staging/output/kernel.elf" > "$staging/output/disassembly.kernel.S"
        arm-none-eabi-objcopy --remove-section=.user_image "$staging/output/kernel.elf" "$staging/output/symbols/kernel.symbols.elf"

        # Replace the complete set, removing obsolete output names and layouts.
        rm -rf output
        mv "$staging/output" output
    )
}

clean(){
    cargo clean
    rm -rf output
}

profile="${1:-dev}"
case "$profile" in
    clean)
        if [ "$#" -ne 1 ]; then usage; exit 2; fi
        clean
        exit 0
        ;;
    dev|release) ;;
    *) usage; exit 2;;
esac

shift
board=""
for argument in "$@"; do
    case "$argument" in
        -*) usage; exit 2 ;;
        *)
            if [ -n "$board" ]; then usage; exit 2; fi
            board="$argument"
            ;;
    esac
done
# Default board is Pico
board="${board:-${PICO_BOARD:-pico}}"

build "$profile" "$board"
