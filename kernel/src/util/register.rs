pub struct Register {
    address: *mut u32,
}

impl Register {
    pub fn new(addr: u32) -> Self {
        Self {
            address: (addr) as *mut u32,
        }
    }

    pub fn write(&self, val: u32) {
        unsafe {
            self.address.write_volatile(val);
        }
    }

    pub fn read(&self) -> u32 {
        unsafe {
            return self.address.read_volatile();
        }
    }

    // fxxking bitset - i cant understand any of them before
    // thx for glorious assistance of gpt-5.6-luna

    /// universal method for read-modify-write routine
    /// ## Bitset universal trick:
    /// ``value = (value & !field_mask) | ((field_value << field_shift) & field_mask)``
    /// ### Where:
    /// - ``(value & !field_mask)``: clears out the bits marked by field_mask from original value
    /// - ``... | ...``: sets new value for cleared bits
    /// - ``(field_value << field_shift)``: aligns the value to be set to the right position
    /// - `` ... & field_mask)``: guarantees that only the field's bits survive
    /// ### Example:
    /// For setting a reg's \[7:5\] to value 0x7, the exact trick would be
    /// - value: \<reg value\> (from arguments of the closure)
    /// - field_mask: 0b1110 0000 (0xE0)
    /// - field_value: 0x7
    /// - field_shift: 0x5 (lowest digit of the desired position)
    /// - final formula: (value & !0xE0) | ((0x7 << 0x5) & 0xE0)
    pub fn modify(&self, func: impl Fn(u32) -> u32) {
        self.write(func(self.read()));
    }

    pub fn bit_get(&self, shift: u32) -> bool {
        let mask: u32 = 0x1 << shift;
        return self.read() & mask != 0;
    }

    /// An trimmed version of field_set - without the need of width
    pub fn bit_set(&self, shift: u32, state: bool) {
        self.modify(|val| {
            let mask: u32 = 0x1 << shift;
            return (val & !mask) | ((u32::from(state) << shift) & mask);
        })
    }

    pub fn field_get(&self, high_bit: u32, low_bit: u32) -> u32 {
        let width = high_bit - low_bit + 1;
        let mask = ((1 << width) - 1) << low_bit;
        return (self.read() & mask) >> low_bit;
    }

    /// An easy-to-use method of the bitset universal trick
    /// ### Internals:
    /// the `field_mask` item is produced with this formula: `((1 << width) - 1) << low_bit`
    /// where width is `high_bit - low_bit + 1`
    /// #### Calculate the `field_mask`
    /// - `(1 << width)` puts 1 to the low_bit
    /// - `... - 1` sets bits [low_bit - 1:0] to 1, to match the length of bit field
    /// - `... << low_bit` aligns these bit to the right position
    pub fn field_set(&self, high_bit: u32, low_bit: u32, state: u32) {
        self.modify(|val| {
            let width = high_bit - low_bit + 1;
            let mask = ((1u32 << width) - 1) << low_bit;
            // shift the field value down to its position (low_bit) and keep
            // only the bits that belong to the field, then splice it in.
            return (val & !mask) | ((state << low_bit) & mask);
        })
    }
}
