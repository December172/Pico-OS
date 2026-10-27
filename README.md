# Pico OS
Small RTOS for Raspberry Pico & Pico 2, using Rust

## Goals
- Multitasking
- Basic shell
- Display (2.4" TFT)
- Audio (Speaker-based)
- Gaming!

## Roadmap
- ✅ LED Blink
- ✅ Clock / Timer 
- ✅ Task Scheduler
- 🛠️ Privileged Kernel Model
- 🛠️ Heap allocation
- 🛠️ UART Output
- ❌ Displaying
- ❌ Basic Shell

## Build

Requires nightly Rust with `rust-src`, Bash, Python 3.9+, ARM GNU
`objcopy`/`objdump`, and picotool. The bundled picotool supports Linux x86-64;
set `PICOTOOL=/path/to/picotool` to use another installation.

Run from the repository root:

```sh
bash tools/build.sh dev pico       # Debug
bash tools/build.sh release pico   # Release
bash tools/build.sh clean          # Remove build artifacts
```

The board defaults to `pico`. Set `PICO_BOARD` or
use different board names as second argument
to select another board. For offline builds with cached
dependencies, set `CARGO_NET_OFFLINE=true`.

The script builds the base user image and links it into the kernel.
Final artifacts are written to `output/`:

```text
output/
├── firmware.elf
├── firmware.uf2
├── disassembly.kernel.S
├── disassembly.user.<image-name>.S
└── symbols/
    ├── firmware.symbols.elf
    └── <image-name>.elf
```

Each successful build replaces the previous output. Intermediate files stay
in Cargo's target directory, including when `CARGO_TARGET_DIR` is set.

To debug, build Debug, BOOTSEL-flash `output/firmware.uf2`, then press F5
in VS Code. F5 does not build or flash. Release builds strip symbols.

See [tools/README.md](tools/README.md) for board configuration and tooling details.

## Development


### Add a new board with supported chips

Copy an existing configuration to `boards/<name>.toml`, for example:

```sh
cp boards/pico.toml boards/my-board.toml
```

Edit its chip and memory sizes (all sizes are bytes):

```toml
chip = "RP2040"               # Choose it from supported chips; currently only RP2040 is supported.
flash-size = 0x00400000       # 4 MiB total external flash
user-flash-size = 0x00040000  # 256 KiB reserved for the user image
user-ram-size = 0x00010000    # 64 KiB reserved for user .data/.bss
```

Then build with `bash tools/build.sh dev my-board` or
`PICO_BOARD=my-board bash tools/build.sh dev`. 
User Flash/RAM partitions occupy the end of their respective memories. Their
sizes must be powers of two, at least 256 bytes, smaller than total memory,
and their starting addresses must align to their sizes.

`chip` selects `cfg(board_chip = "RP2040")` through `build.rs`.
You can use `#[cfg(board_chip = "RP2040")]` to specify chips in task code.

## Acknowledgement
- ChatGPT Codex for architecture design, code refining, and other assistance.
- Deepseek-v4.1-flash for constants generating tools
