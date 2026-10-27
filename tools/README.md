## Build tools and file placement

See [Build](../README.md#build) for prerequisites, build commands, output
files, and the flashing workflow.

- `tools/build.sh`: build user images, pack and link them into the kernel,
  then publish firmware and matching debugger files.
- `tools/pack_user_image.py`: convert user ELFs into a checksummed binary
  and a linkable object, preserving each image's header, CRC, and Flash position.
- `build-support/`: shared Rust build code for memory layouts, linker inputs,
  and ABI constants.
- `boards/`: board chip and memory configurations.
- `user/images.toml`: Flash/RAM slots for user images.
- `user/linkers/` and `kernel/linkers/`: shared user and kernel linker scripts.
  Image-specific scripts stay in their image crates.

Keep command-line tools under `tools/` and Cargo `build.rs` entry points
in their respective crates.

The mandatory base image supplies the idle task. The build script uses the
same board and profile for the user images and kernel. Packed intermediates
are stored at
`<cargo-target-dir>/pico-images/<board>/<debug|release>/base-image.{bin,o}`.
Build or conversion failures preserve the previous published output.

GDB uses `output/firmware.elf` as its executable and loads symbols from
`output/symbols/`. The kernel symbol ELF excludes `.user_image` to prevent
overlapping addresses from hiding user source lines. Files under
`output/symbols/` are for debugging only.

The supplied rust-analyzer settings expect
`target/pico-images/pico/debug/base-image.o`. Build once to generate it;
adjust `PICO_BOARD` and `PICO_USER_IMAGE` in the workspace settings when
using another board or Cargo target directory.

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
