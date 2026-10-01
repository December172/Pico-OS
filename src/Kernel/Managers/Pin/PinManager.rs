use crate::HAL::Pin::*;

use crate::Native::Constants::Config::MAX_CAPABILITY_SOURCES;
use crate::Native::Constants::Config::MAX_PIN;

pub struct PinManager {
    usedPin: u64,
    /// without `+ 1`, the MAX_PIN cannot be indexed - causes index out of bounds expection
    pinCapLookup : [[Option<&'static PinCapability>; MAX_CAPABILITY_SOURCES]; MAX_PIN as usize + 1],
    pinCapCount: [u8; MAX_PIN as usize + 1],
}

impl PinManager {
    pub fn new() -> Self {
        Self {
            usedPin: 0,
            pinCapLookup: [[None; MAX_CAPABILITY_SOURCES]; MAX_PIN as usize + 1],
            pinCapCount: [0; MAX_PIN as usize + 1],
        }
    }

    pub fn registerCapabilities(&mut self, source: &'static [PinCapability]) {
        for (capabilityId, capability) in source.iter().enumerate() {
            let pin = capability.pin as usize;
            let slot = self.pinCapCount[pin] as usize;
            if slot >= MAX_CAPABILITY_SOURCES {
                panic!("Pin capabilities registry full")
            }

            self.pinCapLookup[pin][slot] = Some(capability);

            self.pinCapCount[pin] += 1;
        }
    }

    pub fn supports(&self, pin: u32, function: PinFunction, periBlock: u32) -> bool {
        if pin == PINOUT_UNUSED || pin > MAX_PIN {
            return false;
        }
        let pinCap = self.pinCapLookup[pin as usize];
        for cap in pinCap.iter().flatten() {
            if cap.function == function && cap.peripherialBlock == periBlock {
                return true;
            }
        }
        return true;
    }

    pub fn isUsed(&self, n: u32) -> bool {
        return (self.usedPin & (1 << n)) != 0;
    }

    // TODO: implement batchClaim() / batchRelease() when heap allocation is available (Vec<> based)
    pub fn claim(&mut self, n: u32) -> Option<Pin>{
        if self.isUsed(n) || n > MAX_PIN {
            return None;
        }
        self.usedPin |= 1u64 << n;
        return Some(Pin::new(n));
    }

    pub fn release(&mut self, pin: Pin) {
        self.usedPin &= !(1u64 << pin.get());
    }
}