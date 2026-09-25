pub struct Register {
    address : *mut u32
}

impl Register {
    pub fn new(addr : u32) -> Self {
        Self {
            address : (addr) as *mut u32,
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
    /// ``value = (value & !fieldMask) | ((fieldValue << fieldShift) & fieldMask)``
    /// ### Where:
    /// - ``(value & !fieldMask)``: clears out the bits marked by fieldMask from original value
    /// - ``... | ...``: sets new value for cleared bits
    /// - ``(fieldValue << fieldShift)``: aligns the value to be set to the right position
    /// - `` ... & fieldMask)``: guarantees that only the field's bits survive
    /// ### Example:
    /// For setting a reg's \[7:5\] to value 0x7, the exact trick would be
    /// - value: \<reg value\> (from arguments of the closure)
    /// - fieldMask: 0b1110 0000 (0xE0)
    /// - fieldValue: 0x7
    /// - fieldShift: 0x5 (lowest digit of the desired position)
    /// - final formula: (value & !0xE0) | ((0x7 << 0x5) & 0xE0)
    pub fn modify(&self, func: impl Fn(u32) -> u32) {
        self.write(func(self.read()));
    }

    pub fn bitGet(&self, shift: u32) -> bool {
        let mask: u32 = 0x1 << shift;
        return self.read() & mask != 0; 
    }

    /// An trimmed version of fieldSet - without the need of width
    pub fn bitSet(&self, shift: u32, state: bool) {
        self.modify(|val| {
            let mask: u32 = 0x1 << shift;
            return (val & !mask) | ((u32::from(state) << shift) & mask);
        })
    }

    pub fn fieldGet(&self, highBit: u32, lowBit: u32) -> u32 {
        let width = highBit - lowBit + 1;
        let mask = ((1 << width) - 1) << lowBit;
        return (self.read() & mask) >> lowBit;
    }

    /// An easy-to-use method of the bitset universal trick
    /// ### Internals:
    /// the `fieldMask` item is produced with this formula: `((1 << width) - 1) << lowBit`
    /// where width is `highBit - lowBit + 1`
    /// #### Calculate the `fieldMask`
    /// - `(1 << width)` puts 1 to the lowBit
    /// - `... - 1` sets bits [lowBit - 1:0] to 1, to match the length of bit field
    /// - `... << lowBit` aligns these bit to the right position
    pub fn fieldSet(&self, highBit: u32, lowBit: u32, state: u32) {
        self.modify(|val| {
            let width = highBit - lowBit + 1;
            let mask = ((1 << width) - 1) << lowBit;
            return (val & !mask) | ((state << mask));
        })
    }
}