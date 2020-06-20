use super::bus::BusDevice;

const EXP1_BASE_ADDR_PORT: u32 = 0x0;
const EXP2_BASE_ADDR_PORT: u32 = 0x4;
const EXP1_DELAY_PORT: u32 = 0x8;
const EXP3_DELAY_PORT: u32 = 0xC;
const BIOS_DELAY_PORT: u32 = 0x10;
const SPU_DELAY_PORT: u32 = 0x14;
const CDROM_DELAY_PORT: u32 = 0x18;
const EXP2_DELAY_PORT: u32 = 0x1C;
const COM_DELAY_PORT: u32 = 0x20;

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
    fn read32(&mut self, addr: u32) -> u32 {
        // return no-ops for now
        match addr {
            EXP1_BASE_ADDR_PORT => 0x1F00_0000,
            EXP2_BASE_ADDR_PORT => 0x1F80_2000,
            EXP1_DELAY_PORT | EXP3_DELAY_PORT | BIOS_DELAY_PORT | SPU_DELAY_PORT
            | CDROM_DELAY_PORT | EXP2_DELAY_PORT | COM_DELAY_PORT => {
                todo!("Read: Other control ports unimplemented")
            }
            _ => panic!("Unsupported memory IO port: 0x{:08X}", addr),
        }
    }
    fn peek32(&self, addr: u32) -> Option<u32> {
        Some(match addr {
            EXP1_BASE_ADDR_PORT => 0x1F00_0000,
            EXP2_BASE_ADDR_PORT => 0x1F80_2000,
            _ => todo!("Peek: Other control ports unimplemented"),
        })
    }
    fn write32(&mut self, addr: u32, data: u32) {
        match addr {
            EXP1_BASE_ADDR_PORT => {
                if data != 0x1F00_0000 {
                    panic!("Attempt to change EXP1 base address!")
                }
            }
            EXP2_BASE_ADDR_PORT => {
                if data != 0x1F80_2000 {
                    panic!("Attempt to change EXP1 base address!")
                }
            }
            _ => {
                println!(
                    "[MemoryController] Delay port unimplemented: 0x{:02X}. Skipping",
                    addr
                );
            }
        }
    }
}
