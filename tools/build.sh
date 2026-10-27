#!/bin/bash

# Resolve all project-relative paths independently of the caller's directory.
set -euo pipefail
tools_directory="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd -- "$tools_directory/.."

usage(){
    echo "Usage: bash tools/build.sh [dev|release|clean] [board]" >&2
}

build(){
    local board="${2:-${PICO_BOARD:-pico}}"
    local profile="release"
    if [ "$1" == "dev" ]; then
        profile="debug"
    fi

    # Only RP2040 is currently supported. Both build scripts validate the
    # configured chip against this target before compiling either image.
    local target="thumbv6m-none-eabi"
    local picotool="${PICOTOOL:-$tools_directory/picotool}"
    local -a binaries=(pico-base-image)
    local -a commands=(cargo python3 arm-none-eabi-objcopy arm-none-eabi-objdump "$picotool")
    local command
    for command in "${commands[@]}"; do
        if ! command -v "$command" >/dev/null 2>&1; then
            echo "Required build tool not found: $command" >&2
            return 1
        fi
    done

    # Cargo also honors target-dir in config files and CARGO_TARGET_DIR.
    local target_directory
    target_directory="$(cargo metadata --format-version=1 --no-deps |
        python3 -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])')"
    local artifact_directory="$target_directory/$target/$profile"
    local -a images=() packages=()
    local binary
    for binary in "${binaries[@]}"; do
        images+=("$artifact_directory/$binary")
        packages+=(-p "$binary")
    done
    local image_prefix="$target_directory/pico-images/$board/$profile/base-image"

    PICO_BOARD="$board" cargo build "${packages[@]}" --bins \
        --target "$target" -Zbuild-std=core --profile "$1"
    python3 tools/pack_user_image.py "${images[@]}" --output-prefix "$image_prefix"
    PICO_BOARD="$board" PICO_USER_IMAGE="$image_prefix.o" \
        cargo pico2040 --profile "$1"

    # Publish the final artifacts only after every conversion has succeeded.
    (
        # Stage under Cargo's target directory; publish firmware and matching
        # debugger symbols together, independently of the build-cache location.
        staging="$(mktemp -d "$target_directory/.pico-build-XXXXXX")"
        trap 'rm -rf -- "$staging"' EXIT
        mkdir -p "$staging/output/symbols"
        cp "$artifact_directory/kernel" "$staging/output/firmware.elf"
        "$picotool" uf2 convert "$staging/output/firmware.elf" "$staging/output/firmware.uf2"
        arm-none-eabi-objdump -d -Mforce-thumb "$staging/output/firmware.elf" \
            > "$staging/output/disassembly.kernel.S"
        # The packed section overlaps the independently linked user ELFs and
        # hides their source locations in GDB. Keep it only in the firmware ELF.
        arm-none-eabi-objcopy --remove-section=.user_image \
            "$staging/output/firmware.elf" "$staging/output/symbols/firmware.symbols.elf"
        for binary in "${binaries[@]}"; do
            cp "$artifact_directory/$binary" "$staging/output/symbols/$binary.elf"
            arm-none-eabi-objdump -d -Mforce-thumb "$staging/output/symbols/$binary.elf" \
                > "$staging/output/disassembly.user.$binary.S"
        done
        # Replace the complete set, removing obsolete output names and layouts.
        rm -rf -- output
        mv -- "$staging/output" output
    )
}

clean(){
    cargo clean
    rm -rf -- output
}

# Reject unsupported arguments before building or cleaning anything.
case "${1:-dev}" in
    clean)
        if [ "$#" -ne 1 ]; then usage; exit 2; fi
        ;;
    dev|release)
        if [ "$#" -gt 2 ]; then
            usage
            exit 2
        fi
        ;;
    *) usage; exit 2;;
esac

case "${1:-dev}" in
    clean) clean ;;
    dev) build dev "${2:-}";;
    release) build release "${2:-}";;
esac
