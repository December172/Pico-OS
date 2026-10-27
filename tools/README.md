## Build tools and file placement

Keep command-line tools under `tools/`. Shared Rust build code lives in the host
crate `build-support/`, and board configurations live under `boards/`.
Keep shared user linker scripts under `user/linkers/`, kernel linker scripts under
`kernel/linkers/`, and image-specific linker scripts in their image crate. Add
workspace-root directories or files only when they need to live at that level.
Cargo's `build.rs` entry points remain in their respective crate directories.

- `build.sh`: run as `bash tools/build.sh dev` or `bash tools/build.sh release`.
  The optional second argument selects `pico` or `pico-16m`; `PICO_BOARD` also works.
  Add `--test-b` as the third argument to include the independent task B image.
- `../build-support/layout.rs` and `../build-support/image.rs`: memory layout and shared
  user-image linker inputs, including ABI constants generated from `pico-abi`.
- `../boards/`: board chip and memory layouts, selected with `PICO_BOARD`
  (default `pico`).
- `pack_user_image.py`: convert user ELFs into checksummed binaries and a linkable object.
  Multiple ELFs retain their individual headers/CRCs and linked Flash positions.
- `../user/linkers/user-sections.ld`: shared user image section layout.

The base image is mandatory: it supplies the idle task. `build.sh` builds the
selected images in one Cargo invocation, packs them once, then links them into
the kernel using the same board and profile. The optional task B image uses the
separate `images.app` Flash/RAM slot from `user/images.toml`.

The script works from any directory and reads Cargo's actual target directory,
including `CARGO_TARGET_DIR` and `build.target-dir` overrides. It writes the
packed images as `output/base-image-<board>-<debug|release>.{bin,o}` or, with
`--test-b`, `output/user-images-<board>-<debug|release>.{bin,o}`. The final
`output/kernel.elf`, `output/kernel.uf2`, and `output/disassembly.S` are updated
after all conversions succeed. `bash tools/build.sh clean` runs `cargo clean`
and removes `output/`, including hidden staging files.

Requirements: Bash, nightly Cargo with the `rust-src` component, Python 3.9+,
ARM GNU `objcopy`/`objdump`, and picotool. The bundled `tools/picotool` is a Linux
x86-64 executable; set `PICOTOOL=/path/to/picotool` to use another installation.
Set `CARGO_NET_OFFLINE=true` for an offline build with cached dependencies.

The current image format uses a 32-byte header, 12-byte task entries, and one 20-byte
RAM segment record referenced by `segment_offset`. It has no segment count
or segment table. The RAM address is still the VMA chosen when linking the
user ELF; runtime RAM allocation and relocation are separate kernel work.
The image format and kernel ABI versions remain 1 until on-board tests pass.
The packer rejects unsupported versions and malformed image metadata before
replacing its outputs. Multi-image packing uses `.user.header` section addresses,
which survive symbol stripping; `--nm` supplies a fallback for custom linker
layouts that only export `__user_image_start`. Gaps are filled with `0xff`, and
each image retains its own CRC.

Run from the workspace root:

```sh
bash tools/build.sh dev pico
bash tools/build.sh dev pico --test-b
bash tools/build.sh release pico-16m
```

The Task B debug configuration in VS Code builds both images and loads both
user ELFs' symbols. See [the task B guide](../user/pico-test-image/README.md) for
RAM probes, progress counters, normal return, and deliberate HardFault commands.

To run the individual steps manually:

```sh
PICO_BOARD=pico cargo build -p pico-base-image \
    --target thumbv6m-none-eabi -Zbuild-std=core --release --offline
python3 tools/pack_user_image.py \
    target/thumbv6m-none-eabi/release/pico-base-image \
    --output-prefix output/base-image
PICO_BOARD=pico PICO_USER_IMAGE=output/base-image.o \
    cargo pico2040 --profile release --offline
```

Release user images build and pack successfully. The current Release kernel
still fails to link because `__aeabi_uread4` and `__aeabi_uread8` are undefined.
Those runtime helpers belong in the consuming ELF's Rust/runtime code.

## Adding a board

Copy an existing configuration to `boards/<name>.toml`, for example:

```sh
cp boards/pico.toml boards/my-board.toml
```

Edit its chip and memory sizes (all sizes are bytes):

```toml
chip = "RP2040"
flash-size = 0x00400000       # 4 MiB total external flash
user-flash-size = 0x00040000  # 256 KiB reserved for the user image
user-ram-size = 0x00010000    # 64 KiB reserved for user data/BSS
```

Then build with `bash tools/build.sh dev my-board` or
`PICO_BOARD=my-board bash tools/build.sh dev`. Board names may contain ASCII
letters, digits, and hyphens. No generated configuration file is required:
each image's build script generates its own `OUT_DIR/memory.ld` from the TOML.

User Flash/RAM partitions occupy the end of their respective memories. Their
sizes must be powers of two, at least 256 bytes, smaller than total memory,
and their starting addresses must align to their sizes. RP2040 has 256 KiB of
main SRAM; its two 4 KiB scratch banks are reserved separately for kernel stacks.
The external Flash size must match the hardware and fit RP2040's 16 MiB XIP range.

`chip` selects `cfg(board_chip = "RP2040")` through `build.rs`; chip Cargo features
are no longer used. Currently only RP2040 is implemented. Cargo's target triple
must be selected before build scripts run, so `.cargo/config.toml` and
`build.sh` still select `thumbv6m-none-eabi`. Supporting another chip requires
its target selection, architecture/driver code, memory layout, and linker
script; adding a new chip name to a board file alone does not implement it.
Build-script cfg values apply only to their own crate, not automatically to
dependencies.

## Constants generation
Per-peripheral constant blocks under `kernel/src/native/constants/<device>/`
are generated from the SVD files in `specs/`:

```sh
python3 tools/gen_constants.py                 # RP2040 field/enum blocks
python3 tools/gen_constants.py RP2040 --offsets # include register addresses
python3 tools/gen_constants.py RP2040 --offsets --reset # discard curated declarations
```

Device selection defaults to `$SVD_DEVICE` or `RP2040`. An explicit argument
takes precedence. RP2350 profiles remain in the tools, but this checkout only
includes `specs/RP2040.svd`; generating another device requires its SVD file.
`--offsets` and `--no-offsets` are mutually exclusive, and `--reset` requires
the register-address pass.

`gen_constants.py` handles all three blocks in one process: it parses the SVD
once, builds the register-address, field-range and enum blocks in memory, then
writes each changed file once. Existing block markers from the former separate
generators are recognized and updated to name `tools/gen_constants.py`.
Unchanged output preserves both contents and modification times.

- **REG** — `<NAME>_BASE` plus register accessors. Runs of registers whose
  addresses form an arithmetic progression collapse into a single `pub fn`
  (`IO_BANK0_GPIO_CTRL(pin)`, `DMA_CH_AL1_CTRL(ch)`, `PIO_INSTR_MEM_OFFSET(n)`,
  `SIO_INTERP0_ACCUM(i)`, ...). Multi-dimension families (channel × alias) pick
  the index dimension that collapses the most registers.
- **FIELD** — `<REG>_<FIELD>_LOW`/`_HIGH`/`_BIT` bit ranges.
- **ENUM** — one constant per `<enumeratedValue>`.

Register generation is enabled with `--offsets`. Use `--reset` together with
`--offsets` for a one-time migration that discards hand-curated declarations.
Existing module lists are preserved unless `--write-mod` is requested.

`config.rs` is intentionally hand-maintained: it holds
board-specific values (GPIO counts, crystal frequency, PLL multipliers) that
cannot be derived from the SVD.

## Pin capabilities generation
`kernel/src/native/drivers/<device>/<peripheral>/pin_capabilities.rs` is generated from
the same SVD files by `tools/gen_pin_capabilities.py`. The datasheet "GPIO
function table" is stored in the SVD as the enumerated values of
`IO_BANK0.GPIO<n>_CTRL.FUNCSEL`, e.g. `GPIO0/F2 = uart0_tx`, so the peripheral,
the signal (`tx` vs `rx`) and the instance (`uart0` vs `uart1`, pwm slice) all
come straight out of the SVD:

```sh
python3 tools/gen_pin_capabilities.py RP2040         # dry run, prints the files
python3 tools/gen_pin_capabilities.py RP2040 --write # update generated files
python3 tools/gen_pin_capabilities.py RP2040 --check # verify managed files
```

Each peripheral folder gets its own table (`gpio`, `uart`, `spi`, `i2c`, `pwm`),
so `PinManager` can aggregate them independently.

The tool is deliberately side-effect free by default and never clobbers
hand-written code: it only replaces a file that already carries the
`AUTO-GENERATED` marker it writes (use `--force` to override), and it skips
peripheral folders that do not exist yet. Signal names that have no
`HAL::PinFunction` equivalent (JTAG, PIO, clocks, USB muxing, HSTX, ...) are
reported as "unmapped" instead of being emitted.
`--check` also skips curated files without the marker; a successful check does
not validate those files. Unchanged generated files are not rewritten.

## PLL calculator

```sh
python3 tools/vcocalc.py 125
python3 tools/vcocalc.py --low-vco 125
python3 tools/vcocalc.py --lock-refdiv 1 125
```

Frequencies are in MHz. The calculator returns the nearest available result;
check `Achieved` when an exact frequency is required. Frequency arguments must
be finite and positive, and a locked reference divider must respect `--ref-min`.
