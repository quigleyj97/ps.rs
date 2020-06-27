use super::bus::{BusDevice, SizedData};
use log::debug;

const EXP1_BASE_ADDR_PORT: u32 = 0x0;
const EXP2_BASE_ADDR_PORT: u32 = 0x4;
const EXP1_DELAY_PORT: u32 = 0x8;
const EXP3_DELAY_PORT: u32 = 0xC;
const BIOS_DELAY_PORT: u32 = 0x10;
const SPU_DELAY_PORT: u32 = 0x14;
const CDROM_DELAY_PORT: u32 = 0x18;
const EXP2_DELAY_PORT: u32 = 0x1C;
const COM_DELAY_PORT: u32 = 0x20;

// todo: I want to move these into a separate interrupt controller module
const I_STAT_PORT: u32 = 0x70;
const I_MASK_PORT: u32 = 0x74;

/// Interface for setting MMC parameters and read delay timings.
///
/// The PSX doesn't actually have a proper MMC, so writes to the BASE_ADDR ports
/// are thought to be no-ops.
///
/// TODO: Confirm the above- this is the assumption in Mednafen
pub struct MemoryController {}

impl MemoryController {
    pub fn new() -> MemoryController {
        MemoryController {}
    }
}

impl BusDevice for MemoryController {
    fn read<T: SizedData>(&mut self, addr: u32) -> T {
        // TODO: bus sizes that aren't 32-bit
        if T::width() != 4 {
            todo!("Smaller bus reads in MemoryController");
        }
        // return no-ops for now
        T::from_le_byteslice(
            &(match addr {
                EXP1_BASE_ADDR_PORT => 0x1F00_0000u32,
                EXP2_BASE_ADDR_PORT => 0x1F80_2000u32,
                EXP1_DELAY_PORT | EXP3_DELAY_PORT | BIOS_DELAY_PORT | SPU_DELAY_PORT
                | CDROM_DELAY_PORT | EXP2_DELAY_PORT | COM_DELAY_PORT => {
                    todo!("Read: Other control ports unimplemented")
                }
                I_MASK_PORT | I_STAT_PORT => {
                    debug!(target: "memctrl", "Interrupts unimplemented, returning 0");
                    0
                }
                _ => panic!("Unsupported memory IO port: ${:08X}", addr),
            })
            .to_le_bytes(),
        )
    }

    fn peek<T: SizedData>(&self, addr: u32) -> Option<T> {
        if T::width() != 4 {
            todo!("Smaller bus reads in MemoryController");
        }
        Some(T::from_le_byteslice(
            &(match addr {
                EXP1_BASE_ADDR_PORT => 0x1F00_0000u32,
                EXP2_BASE_ADDR_PORT => 0x1F80_2000u32,
                _ => todo!("Peek: Other control ports unimplemented"),
            })
            .to_le_bytes(),
        ))
    }

    fn write<T: SizedData>(&mut self, addr: u32, data: T) {
        match addr {
            EXP1_BASE_ADDR_PORT => {
                if data != T::from_u32(0x1F00_0000) {
                    panic!("Attempt to change EXP1 base address!")
                }
            }
            EXP2_BASE_ADDR_PORT => {
                if data != T::from_u32(0x1F80_2000) {
                    panic!("Attempt to change EXP1 base address!")
                }
            }
            I_MASK_PORT => {
                if data != T::from_u32(0x0) {
                    todo!("Interrupt controller");
                }
                debug!(target: "memctrl", "Disabling write to I_MASK");
            }
            _ => {
                debug!(
                    target: "memctrl",
                    "Delay port unimplemented: ${:02X}. Skipping", addr
                );
            }
        }
    }
}
