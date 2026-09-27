# Pico OS
Small RTOS for Raspberry Pico & Pico 2, using Rust

## Goals
- Multitasking
- Basic shell
- Display (2.4" TFT)
- Audio (Speaker-based)
- Gaming!

## Constants generation
Every per-peripheral constants file under `src/Native/Constants/<DEVICE>/` is
auto-generated from the SVD files in `specs/` by the Python tools in `tools/`:

```sh
python tools/gen_constants.py RP2350          # full regeneration
python tools/gen_constants.py RP2040 --offsets # regenerate existing files
python tools/gen_constants.py RP2040 --offsets --reset  # drop hand-written decls first
```

Each file is built from three delimited blocks, all regenerated in place so
running the tools twice is a no-op:

- **REG** — `<NAME>_BASE` plus register accessors. Runs of registers whose
  addresses form an arithmetic progression collapse into a single `pub fn`
  (`IO_BANK0_GPIO_CTRL(pin)`, `DMA_CH_AL1_CTRL(ch)`, `PIO_INSTR_MEM_OFFSET(n)`,
  `SIO_INTERP0_ACCUM(i)`, ...). Multi-dimension families (channel × alias) pick
  the index dimension that collapses the most registers.
- **FIELD** — `<REG>_<FIELD>_LOW`/`_HIGH`/`_BIT` bit ranges.
- **ENUM** — one constant per `<enumeratedValue>`.

The active device is selected with the `SVD_DEVICE` environment variable
(default `RP2040`). `tools/gen_register_offsets.py` refuses to overwrite a file
whose addresses are declared outside its own block until the migration flag
(`--reset`, or `SVD_FORCE_OFFSETS=1`) is given.

`Config.rs` is the single intentionally hand-maintained file: it holds
board-specific values (GPIO counts, crystal frequency, PLL multipliers) that
cannot be derived from the SVD.

## Acknowledgement
GPT-5.6-Luna for architecture design
Deepseek-v4.1-flash for constants generating tools
