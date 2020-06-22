//! MIPS-I coprocessor0, as implemented on the PSX R3000
//!
//! I can't find very much reliable information about the cop0, but looking at
//! a few other emulators it seems that not much effort is put into emulating
//! this accurately. That tracks with the fact that the PSX runs all
//! instructions in "kernel" mode, meaning the cop0 is not particularly useful
//! for most PSX games, aside from exception management.
//!
//! Therefore, this implementation doesn't do very much either.
//!
//! Presumably cop0 emulation may be required for some titles, and it might have
//! been useful for Net Yaroze and debug builds.

use log::debug;

pub struct Cop0 {
    /// R12 status register
    sr: u32,
}

//#region SR Flags
/// Flag set when memory ops should only hit the cache instead of the bus
const CACHE_ISOLATE: u32 = 0x0001_0000;
//#endregion

//#region COP0 register addresses
const BPC_IDX: usize = 3;
const BDA_IDX: usize = 5;
// TODO: clarify what this register is, and whether it's important
const MYSTERY_IDX: usize = 6;
const DCIC_IDX: usize = 7;
const BDAM_IDX: usize = 9;
const BPCM_IDX: usize = 11;
const SR_IDX: usize = 12;
const CAUSE_IDX: usize = 13;
//#endregion

impl Cop0 {
    pub fn new() -> Cop0 {
        // I'm guessing at these power-on values- I actually don't know
        Cop0 { sr: 0 }
    }

    pub fn is_cache_isolated(&self) -> bool {
        return (self.sr & CACHE_ISOLATE) > 0;
    }

    pub fn mtc(&mut self, regidx: usize, data: u32) {
        match regidx {
            SR_IDX => self.sr = data,
            // these registers are for hardware breakpoints, ignore them for now
            BPC_IDX | BDA_IDX | MYSTERY_IDX | DCIC_IDX | BDAM_IDX | BPCM_IDX => {
                debug!(target: "cop0", "MTC to unimplemented breakpoint register {}", regidx);
                // if the written value _isn't_ zero, the game is trying to
                // do something. panic to make it visible
                if data != 0 {
                    panic!(
                        "Attempt to enable hardware breakpoint in cop0: ${:02X} = 0x{:02X}",
                        regidx, data
                    );
                }
            }
            CAUSE_IDX => {
                // same as above
                debug!(target: "cop0", "MTC to unimplemented cause register");
                if data != 0 {
                    panic!("Possible attempt to trigger hardware exception in cop0");
                }
            }
            _ => todo!("MTC0 for register {} = 0x{:08X}", regidx, data),
        }
    }
}
