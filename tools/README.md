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

## Pin capabilities generation
`src/Native/Drivers/<DEVICE>/<Peripheral>/PinCapabilities.rs` is generated from
the same SVD files by `tools/gen_pin_capabilities.py`. The datasheet "GPIO
function table" is stored in the SVD as the enumerated values of
`IO_BANK0.GPIO<n>_CTRL.FUNCSEL`, e.g. `GPIO0/F2 = uart0_tx`, so the peripheral,
the signal (`tx` vs `rx`) and the instance (`uart0` vs `uart1`, pwm slice) all
come straight out of the SVD:

```sh
python tools/gen_pin_capabilities.py RP2040           # dry run, prints the files
python tools/gen_pin_capabilities.py RP2350 --write   # update the files
python tools/gen_pin_capabilities.py RP2350 --check   # verify, non-zero on stale
```

Each peripheral folder gets its own table (`GPIO`, `UART`, `SPI`, `I2C`, `PWM`),
so `PinManager` can aggregate them independently.

The tool is deliberately side-effect free by default and never clobbers
hand-written code: it only replaces a file that already carries the
`AUTO-GENERATED` marker it writes (use `--force` to override), and it skips
peripheral folders that do not exist yet. Signal names that have no
`HAL::PinFunction` equivalent (JTAG, PIO, clocks, USB muxing, HSTX, ...) are
reported as "unmapped" instead of being emitted.