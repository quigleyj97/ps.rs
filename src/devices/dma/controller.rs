//! The PSX DMA controller

use super::structs::DmaChannel;
use crate::devices::bus::{BusDevice, SizedData};
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
    /// DMA channel control registers
    channels: [DmaChannel; 7],
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
            channels: [DmaChannel::from(0); 7],
        }
    }
}

impl BusDevice for DmaController {
    fn read<T: SizedData>(&mut self, addr: u32) -> T {
        if T::width() != 4 {
            todo!("Other bit widths");
        }
        let major = (addr & 0x70) >> 4;
        let minor = addr & 0x0F;
        match major {
            0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                let channel = &self.channels[major as usize];
                match minor {
                    0x0 => todo!(),
                    0x4 => todo!(),
                    0x8 => T::from_u32(**channel),
                    0xC => todo!(),
                    _ => unreachable!(),
                }
            }
            7 => match minor {
                0x0 => T::from_u32(self.control),
                0x4 => T::from_u32(self.interrupt),
                0x8 => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 1");
                    T::from_u32(self.unknown_1)
                }
                0xC => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 2");
                    T::from_u32(self.unknown_2)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn peek<T: SizedData>(&self, addr: u32) -> Option<T> {
        if T::width() != 4 {
            todo!("Other bit widths");
        }
        let major = (addr & 0x70) >> 4;
        let minor = addr & 0x0F;
        Some(match major {
            0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                let channel = &self.channels[major as usize];
                match minor {
                    0x0 => todo!(),
                    0x4 => todo!(),
                    0x8 => T::from_u32(**channel),
                    0xC => todo!(),
                    _ => unreachable!(),
                }
            }
            7 => match minor {
                0x0 => T::from_u32(self.control),
                0x4 => T::from_u32(self.interrupt),
                0x8 => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 1");
                    T::from_u32(self.unknown_1)
                }
                0xC => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 2");
                    T::from_u32(self.unknown_2)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
    }

    fn write<T: SizedData>(&mut self, addr: u32, data: T) {
        if T::width() != 4 {
            todo!("Other bit widths");
        }
        let major = (addr & 0x70) >> 4;
        let minor = addr & 0x0F;
        match major {
            0 | 1 | 2 | 3 | 4 | 5 | 6 => match minor {
                0x0 => todo!(),
                0x4 => todo!(),
                0x8 => self.channels[major as usize] = DmaChannel::from(data.to_u32()),
                0xC => todo!(),
                _ => unreachable!(),
            },
            7 => match minor {
                0x0 => self.control = data.to_u32(),
                0x4 => self.interrupt = data.to_u32(),
                0x8 => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 1");
                    self.unknown_1 = data.to_u32()
                }
                0xC => {
                    debug!(target: "dma", "Attempt to use unknown DMA register 2");
                    self.unknown_2 = data.to_u32()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
