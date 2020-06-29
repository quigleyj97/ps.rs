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

use crate::devices::cpu::CpuR3000;
use crate::utils::cpustructs::{Exception, Instruction, MagicAddress};
use log::debug;

pub struct Cop0 {
    /// R12 status register
    sr: u32,
    /// R13 Cause register
    cause: u32,
    /// R14 Exception return address
    epc: u32,
}

//#region SR Flags
/// Flag set when memory ops should only hit the cache instead of the bus
const CACHE_ISOLATE: u32 = 0x0001_0000;
const BOOT_EXC_VECTORS: u32 = 0x0040_0000;
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
const EPC_IDX: usize = 14;
//#endregion

impl Cop0 {
    pub fn new() -> Cop0 {
        // I'm guessing at these power-on values- I actually don't know
        Cop0 {
            sr: 0,
            cause: 0,
            epc: 0,
        }
    }

    pub fn is_cache_isolated(&self) -> bool {
        return (self.sr & CACHE_ISOLATE) > 0;
    }

    pub fn is_bev(&self) -> bool {
        return (self.sr & BOOT_EXC_VECTORS) > 0;
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
                if data != 0 {
                    panic!("Possible attempt to trigger hardware exception in cop0");
                }
                self.cause = data;
            }
            EPC_IDX => {
                self.epc = data;
            }
            _ => todo!("MTC0 for register {} = 0x{:08X}", regidx, data),
        }
    }

    pub fn mfc(&mut self, regidx: usize) -> u32 {
        match regidx {
            SR_IDX => self.sr,
            CAUSE_IDX => self.cause,
            EPC_IDX => self.epc,
            _ => todo!("Unhandled read from cop0 {} register", regidx),
        }
    }
}

/// Setup state for an exception handler, and return the next CPU address
pub fn handle_exception(cpu: &mut CpuR3000, exc: Exception, pc: u32, is_delay_slot: bool) -> u32 {
    let cop0 = &mut cpu.cop0;
    // setup the cause register
    cop0.cause = 0 | ((exc as u32) << 2);

    // advance the interrupt enable bits
    let mode = cop0.sr & 0x3F;
    cop0.sr &= !0x3f;
    cop0.sr |= (mode << 2) & 0x3F;

    // set the return address
    cop0.epc = pc;

    if is_delay_slot {
        // we need to correct the EPC and cause register to reflect that we are
        // inside a delay slot
        cop0.cause |= 0x8000_0000;
        cop0.epc = cop0.epc.wrapping_sub(4);
    }

    let is_tlb_exc = exc == Exception::TLBModification
        || exc == Exception::TLBLoad
        || exc == Exception::TLBStore;

    match (is_tlb_exc, cop0.is_bev()) {
        (false, false) => MagicAddress::MiscException as u32,
        (false, true) => MagicAddress::MiscExceptionBev as u32,
        (true, false) => MagicAddress::TLBMiss as u32,
        (true, true) => MagicAddress::TLBMissBev as u32,
    }
}

pub fn handle_cop_instr(cpu: &mut CpuR3000, instr: Instruction) {
    match instr.funct() {
        0b010000 => {
            // RFE
            let cop = &mut cpu.cop0;
            let mode = cop.sr & 0x3F;
            cop.sr &= !0x3F;
            cop.sr |= mode >> 2;
        }
        0b001000 => todo!("TLBP"),  // TLB probe
        0b000001 => todo!("TLBR"),  // Read indexed TLB entry
        0b000010 => todo!("TLBWI"), // Write indexed TLB entry
        // "Write Random" TLB entry
        //
        // ...no, I have no clue what that means either
        0b000110 => todo!("TLBWR"),
        _ => panic!("Unknown COP0 instruction {:08X}", *instr),
    }
}
