//! The PSX DMA controller

use super::bus::{BusDevice, SizedData};
use log::debug;

pub struct DmaController {
    /// Control register
    control: u32,
    /// DMA Interrupt Register
    interrupt: u32,
    /// An unknown register at F8, according to no$psx
    unknown_1: u32,
    /// An unknown register at FC, according to no$psx
    unknown_2: u32,
}

impl DmaController {
    pub fn new() -> DmaController {
        DmaController {
            // No$psx list this as the reset value for the control register
            control: 0x0765_4321,
            // the rest of these are guesses
            interrupt: 0,
            unknown_1: 0,
            unknown_2: 0,
        }
    }
}

impl BusDevice for DmaController {
    fn read<T: SizedData>(&mut self, addr: u32) -> T {
        if T::width() != 4 {
            todo!("Other bit widths");
        }
        if addr > 0x6F {
            // this is a control register
            return match addr {
                0x70 => T::from_u32(self.control),
                0x74 => T::from_u32(self.interrupt),
                0x78 => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 1");
                    T::from_u32(self.unknown_1)
                }
                0x7C => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 2");
                    T::from_u32(self.unknown_2)
                }
                _ => unreachable!(),
            };
        }
        // this is a DMA port
        todo!("DMA ports");
    }

    fn peek<T: SizedData>(&self, addr: u32) -> Option<T> {
        if T::width() != 4 {
            todo!("Other bit widths");
        }
        if addr > 0x6F {
            // this is a control register
            return Some(match addr {
                0x70 => T::from_u32(self.control),
                0x74 => T::from_u32(self.interrupt),
                0x78 => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 1");
                    T::from_u32(self.unknown_1)
                }
                0x7C => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 2");
                    T::from_u32(self.unknown_2)
                }
                _ => unreachable!(),
            });
        }
        // this is a DMA port
        todo!("DMA ports");
    }

    fn write<T: SizedData>(&mut self, addr: u32, data: T) {
        if T::width() != 4 {
            todo!("Other bit widths");
        }
        if addr > 0x6F {
            // this is a control register
            return match addr {
                0x70 => self.control = data.to_u32(),
                0x74 => self.interrupt = data.to_u32(),
                0x78 => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 1");
                    self.unknown_1 = data.to_u32()
                }
                0x7C => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 2");
                    self.unknown_2 = data.to_u32()
                }
                _ => unreachable!(),
            };
        }
        // this is a DMA port
        todo!("DMA port $+{:02X} = 0x{:08X}", addr, data);
    }
}
