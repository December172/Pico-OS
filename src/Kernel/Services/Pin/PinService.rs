use crate::HAL::Pin::Pin;
use crate::Kernel::Services::Service::Service;
use crate::Kernel::Services::ServiceID::ServiceID;

pub struct PinService {
    usedPin : u32
}

impl PinService {
    pub fn new() -> Self {
        Self {
            usedPin : 0
        }
    }

    pub fn isUsed(&self, n: u8) -> bool {
        return (self.usedPin & (1 << n)) != 0;
    }

    pub fn claim(&mut self, n: u8) -> Option<Pin>{
        if self.isUsed(n) {
            return None;
        }
        self.usedPin |= 1 << n;
        return Some(Pin::new(n));
    }

    pub fn release(&mut self, pin: Pin) {
        self.usedPin ^= 1 << pin.get();
    }
}

impl Service for PinService {
    fn getID(&self) -> ServiceID {
        return ServiceID::PinService;
    }
}