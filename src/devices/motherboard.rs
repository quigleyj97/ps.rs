use crate::devices::bus::BusDevice;
use crate::devices::cpu;
use crate::devices::memctrl::MemoryController;
use crate::devices::rom::Rom;
use crate::utils::memorymap::{map_device, Device};
use log::debug;

/// This represents the system motherboard.
///
/// This owns all devices, and updates devices with respect to a main clock.
pub struct Motherboard {
    bios: Rom,
    memctrl: MemoryController,
    pub cpu: cpu::CpuR3000,
}

impl Motherboard {
    pub fn tick(&mut self) {
        cpu::exec(self);
    }

    pub fn new(bios: Vec<u8>) -> Motherboard {
        return Motherboard {
            bios: Rom::from_buf(bios),
            cpu: cpu::CpuR3000::new(),
            memctrl: MemoryController::new(),
        };
    }
}

impl BusDevice for Motherboard {
    fn read32(&mut self, addr: u32) -> u32 {
        let (_seg, dev, local_addr) = map_device(addr);
        if addr % 4 != 0 {
            panic!("Unaligned memory access: ${:08X}", addr);
        }
        match dev {
            // Device::RAM => {}
            // Device::Expansion1 => {}
            // Device::Scratch => {}
            Device::IO => self.memctrl.read32(local_addr),
            // Device::Expansion2 => {}
            // Device::Expansion3 => {}
            Device::BIOS => self.bios.read32(local_addr),
            _ => panic!("Unmapped memory: ${:08X}", addr),
            // Device::IOCacheControl => {}
            // Device::None => {}
            // Device::VMemException => {}
        }
    }

    fn peek32(&self, addr: u32) -> Option<u32> {
        let (_seg, dev, local_addr) = map_device(addr);
        if addr % 4 != 0 {
            panic!("Unaligned memory access: ${:08X}", addr);
        }
        match dev {
            // Device::RAM => {}
            // Device::Expansion1 => {}
            // Device::Scratch => {}
            Device::IO => self.memctrl.peek32(local_addr),
            // Device::Expansion2 => {}
            // Device::Expansion3 => {}
            Device::BIOS => self.bios.peek32(local_addr),
            _ => None,
            // Device::IOCacheControl => {}
            // Device::None => {}
            // Device::VMemException => {}
        }
    }

    fn write32(&mut self, addr: u32, data: u32) {
        let (_seg, dev, local_addr) = map_device(addr);
        if addr % 4 != 0 {
            panic!("Unaligned memory access: ${:08X}", addr);
        }
        match dev {
            // Device::RAM => {}
            // Device::Expansion1 => {}
            // Device::Scratch => {}
            Device::IO => self.memctrl.write32(local_addr, data),
            // Device::Expansion2 => {}
            // Device::Expansion3 => {}
            Device::BIOS => panic!(
                "Attempt to write 0x{:08X} to read-only BIOS at ${:08}",
                data, addr
            ),
            Device::IOCacheControl => {
                // todo: implement actual cache control
                debug!(target: "mb",
                    "Write to cache control register ignored: ${:08X} = 0x{:08X}",
                    addr, data
                );
            }
            _ => panic!("Unmapped memory: ${:08X}", addr),
            // Device::None => {}
            // Device::VMemException => {}
        }
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
