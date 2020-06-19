use crate::devices::bus::BusDevice;
use crate::devices::cpu;
use crate::devices::rom::Rom;

/// This represents the system motherboard.
///
/// This owns all devices, and updates devices with respect to a main clock.
pub struct Motherboard {
    _bios: Rom,
    pub cpu: cpu::CpuR3000,
}

impl Motherboard {
    pub fn tick(&mut self) {
        cpu::exec(self);
    }

    pub fn new(bios: Vec<u8>) -> Motherboard {
        return Motherboard {
            _bios: Rom::from_buf(bios),
            cpu: cpu::CpuR3000::new(),
        };
    }
}

impl BusDevice for Motherboard {
    fn read32(&mut self, _addr: u32) -> u32 {
        0
    }

    fn peek32(&self, _addr: u32) -> Option<u32> {
        Some(0)
    }

    fn write32(&mut self, _addr: u32, _data: u32) {
        // no-op
    }
}

impl cpu::WithCpu for Motherboard {
    fn cpu_mut(&mut self) -> &mut cpu::CpuR3000 {
        return &mut self.cpu;
    }

    fn cpu(&self) -> &cpu::CpuR3000 {
        return &self.cpu;
    }
}
