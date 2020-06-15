use crate::devices::rom::Rom;

/// This represents the system motherboard.
///
/// This owns all devices, and updates devices with respect to a main clock.
pub struct Motherboard {
    bios: Rom,
}

impl Motherboard {
    pub fn read32(&mut self, addr: u32) -> u32 {
        0
    }

    pub fn peek32(&self, addr: u32) -> Option<u32> {
        Some(0)
    }

    pub fn write32(&mut self, addr: u32, data: u32) {
        // no-op
    }

    pub fn new(bios: Vec<u8>) -> Motherboard {
        return Motherboard {
            bios: Rom::from_buf(bios),
        };
    }
}
