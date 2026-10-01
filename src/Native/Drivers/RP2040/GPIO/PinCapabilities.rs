// Pin capabilities of the RP2040 GPIO driver.
// Source: RP2040 datasheet, "GPIO function table" (F5 = SIO).
// Bank 0 GPIO0..GPIO29 can all be used as plain software GPIO.

use crate::HAL::Pin::PinCapability;
use crate::HAL::Pin::PinFunction;

pub static PIN_CAPABILITIES: &[PinCapability] = &[
    PinCapability { pin:  0, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  1, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  2, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  3, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  4, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  5, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  6, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  7, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  8, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin:  9, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 10, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 11, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 12, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 13, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 14, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 15, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 16, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 17, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 18, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 19, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 20, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 21, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 22, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 23, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 24, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 25, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 26, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 27, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 28, function: PinFunction::GPIO, peripherialBlock: 0 },
    PinCapability { pin: 29, function: PinFunction::GPIO, peripherialBlock: 0 },
];
