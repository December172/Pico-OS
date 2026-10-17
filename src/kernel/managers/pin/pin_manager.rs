use crate::hal::pin::*;

use crate::native::constants::config::MAX_CAPABILITY_SOURCES;
use crate::native::constants::config::MAX_PIN;

pub struct PinManager {
    used_pin: u64,
    /// without `+ 1`, the MAX_PIN cannot be indexed - causes index out of bounds expection
    pin_cap_lookup:
        [[Option<&'static PinCapability>; MAX_CAPABILITY_SOURCES]; MAX_PIN as usize + 1],
    pin_cap_count: [u8; MAX_PIN as usize + 1],
}

impl PinManager {
    pub fn new() -> Self {
        Self {
            used_pin: 0,
            pin_cap_lookup: [[None; MAX_CAPABILITY_SOURCES]; MAX_PIN as usize + 1],
            pin_cap_count: [0; MAX_PIN as usize + 1],
        }
    }

    pub fn register_capabilities(&mut self, source: &'static [PinCapability]) {
        for (capability_id, capability) in source.iter().enumerate() {
            let pin = capability.pin as usize;
            let slot = self.pin_cap_count[pin] as usize;
            if slot >= MAX_CAPABILITY_SOURCES {
                panic!("Pin capabilities registry full")
            }

            self.pin_cap_lookup[pin][slot] = Some(capability);

            self.pin_cap_count[pin] += 1;
        }
    }

    pub fn supports(&self, pin: u32, function: PinFunction, peri_block: u32) -> bool {
        if pin == PINOUT_UNUSED || pin > MAX_PIN {
            return false;
        }
        let pin_cap = self.pin_cap_lookup[pin as usize];
        for cap in pin_cap.iter().flatten() {
            if cap.function == function && cap.peripheral_block == peri_block {
                return true;
            }
        }
        return true;
    }

    pub fn is_used(&self, n: u32) -> bool {
        return (self.used_pin & (1 << n)) != 0;
    }

    // TODO: implement batch_claim() / batch_release() when heap allocation is available (Vec<> based)
    pub fn claim(&mut self, n: u32) -> Option<Pin> {
        if self.is_used(n) || n > MAX_PIN {
            return None;
        }
        self.used_pin |= 1u64 << n;
        return Some(Pin::new(n));
    }

    pub fn release(&mut self, pin: Pin) {
        self.used_pin &= !(1u64 << pin.get());
    }
}
