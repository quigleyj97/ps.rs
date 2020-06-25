use crate::devices::bus::{BusDevice, SizedData};
use crate::devices::cpu;
use crate::devices::memctrl::MemoryController;
use crate::devices::ram::Ram;
use crate::devices::rom::Rom;
use crate::utils::memorymap::{map_device, Device};
use log::debug;

/// This represents the system motherboard.
///
/// This owns all devices, and updates devices with respect to a main clock.
pub struct Motherboard {
    bios: Rom,
    ram: Ram,
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
            ram: Ram::with_size(2 * 1024 * 1024),
            cpu: cpu::CpuR3000::new(),
            memctrl: MemoryController::new(),
        };
    }
}

impl BusDevice for Motherboard {
    fn read<T: SizedData>(&mut self, addr: u32) -> T {
        let (_seg, dev, local_addr) = map_device(addr);
        if !T::is_aligned(addr) {
            panic!("Unaligned memory access: ${:08X}", addr);
        }
        match dev {
            Device::RAM => self.ram.read::<T>(local_addr),
            Device::Expansion1 => {
                // This is the parallel port out the back, which is nominally
                // unplugged. Mednafen and Rustation return all ones here,
                // suggesting that the hardware uses internal pullup resistors
                debug!(target: "cpu", "Attempt to read from parallel port, ignoring");
                T::from_u32(0)
            }
            // Device::Scratch => {}
            Device::MemCtrl => self.memctrl.read::<T>(local_addr),
            Device::SPU => {
                debug!(target: "cpu", "Attempt to read from SPU, ignoring for now");
                T::from_u32(0)
            }
            // Device::Expansion2 => {}
            // Device::Expansion3 => {}
            Device::BIOS => self.bios.read::<T>(local_addr),
            _ => panic!("Unmapped memory: ${:08X}", addr),
            // Device::IOCacheControl => {}
            // Device::None => {}
            // Device::VMemException => {}
        }
    }

    fn peek<T: SizedData>(&self, addr: u32) -> Option<T> {
        let (_seg, dev, local_addr) = map_device(addr);
        if !T::is_aligned(addr) {
            panic!("Unaligned memory access: ${:08X}", addr);
        }
        match dev {
            Device::RAM => self.ram.peek::<T>(local_addr),
            // Device::Expansion1 => {}
            // Device::Scratch => {}
            Device::MemCtrl => self.memctrl.peek::<T>(local_addr),
            Device::SPU => {
                debug!("Attempt to peek from SPU, ignoring for now");
                Some(T::from_u32(0))
            }
            // Device::Expansion2 => {}
            // Device::Expansion3 => {}
            Device::BIOS => self.bios.peek::<T>(local_addr),
            _ => None,
            // Device::IOCacheControl => {}
            // Device::None => {}
            // Device::VMemException => {}
        }
    }

    fn write<T: SizedData>(&mut self, addr: u32, data: T) {
        let (_seg, dev, local_addr) = map_device(addr);
        if !T::is_aligned(addr) {
            panic!("Unaligned memory access: ${:08X}", addr);
        }
        match dev {
            Device::RAM => self.ram.write(local_addr, data),
            // Device::Expansion1 => {}
            // Device::Scratch => {}
            Device::MemCtrl => self.memctrl.write(local_addr, data),
            Device::SPU => {
                debug!(target: "mb", "Attempt to write to SPU, but SPU is unimplemented: ${:08X} = 0x{:08X}", addr, data)
            }
            Device::Expansion2 => {
                debug!(target: "cpu", "Attempt to write to Expansion2: ${:08X} = 0x{:08X}", addr, data);
            }
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
