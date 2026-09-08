use crate::HAL::Pin::*;

pub struct GPIO {
    usedPin : u32;
}

impl GPIO {
    pub fn isUsed<const N: u8>(&self, pin : &Pin<N>) -> bool {
        return (self.usedPin & (1 << N)) != 0;
    }
}