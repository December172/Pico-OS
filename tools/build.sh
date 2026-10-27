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
    local image_prefix="output/base-image-$board-$profile"
    local snapshot_directory="output/$board/$profile"

    PICO_BOARD="$board" cargo build "${packages[@]}" --bins \
        --target "$target" -Zbuild-std=core --profile "$1"
    python3 tools/pack_user_image.py "${images[@]}" --output-prefix "$image_prefix"
    PICO_BOARD="$board" PICO_USER_IMAGE="$image_prefix.o" \
        cargo pico2040 --profile "$1"

    # Publish the final artifacts only after every conversion has succeeded.
    (
        staging="$(mktemp -d output/.pico-build-XXXXXX)"
        trap 'rm -rf -- "$staging"' EXIT
        cp "$artifact_directory/kernel" "$staging/kernel.elf"
        "$picotool" uf2 convert "$staging/kernel.elf" "$staging/kernel.uf2"
        arm-none-eabi-objdump -d -Mforce-thumb "$staging/kernel.elf" > "$staging/disassembly.S"
        # The packed section overlaps the independently linked user ELFs and
        # hides their source locations in GDB. Keep it only in the firmware ELF.
        arm-none-eabi-objcopy --remove-section=.user_image \
            "$staging/kernel.elf" "$staging/kernel-symbols.elf"
        for binary in "${binaries[@]}"; do
            cp "$artifact_directory/$binary" "$staging/$binary.elf"
        done
        # Launch configurations use a matching board/profile snapshot.
        mkdir -p -- "$snapshot_directory"
        cp -- "$staging/"* "$snapshot_directory/"
        mv -- "$staging/"* output/
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
