use crate::devices::bus::BusDevice;
use crate::utils::cpustructs::{Instruction, Mnemonic};
use crate::utils::decode::decode_instruction;

macro_rules! sign_extend {
    ($val: expr) => {{
        (i32::from($val as i16) as u32)
    }};
}

macro_rules! zero_extend {
    ($val: expr) => {{
        (($val as u16) as u32)
    }};
}

macro_rules! op_fn {
    ($mnemonic:ident, ($cpu: ident, $instr: ident), $body: expr) => {
        fn $mnemonic<T: WithCpu + BusDevice>($cpu: &mut T, $instr: Instruction) {
            $body
        }
    };
}

#[derive(Clone, Debug)]
pub struct CpuState {
    /// The CPU registers
    registers: [u32; 32],
    /// The program counter register
    pub pc: u32,
    /// Number of idle cycles to burn to synchronize the CPU with the clock
    ///
    /// Some operations will increase this, for things like reads from memory,
    /// which represent how many cycles the CPU will be blocked executing that
    /// read.
    pub wait: u32,
}

pub const CPU_POWERON_STATE: CpuState = CpuState {
    // from IDX docs
    pc: 0xBFC0_0000,
    // the rest of this is shooting from the hip
    registers: [0u32; 32],
    wait: 0,
};

/// The CPU for the PlayStation
///
/// This CPU is a MIPS ISA with a 5-stage pipeline
pub struct CpuR3000 {
    pub state: CpuState,
    pub cycles: u64,
}

impl CpuR3000 {
    pub fn new() -> CpuR3000 {
        return CpuR3000 {
            state: CPU_POWERON_STATE.clone(),
            cycles: 0,
        };
    }
}

/// A trait for devices that own a CPU, such as the Motherboard
pub trait WithCpu {
    fn cpu_mut(&mut self) -> &mut CpuR3000;
    fn cpu(&self) -> &CpuR3000;
}

/// Burn cycles if the CPU needs to wait, and return whether the CPU is in sync
pub fn tick<T: WithCpu>(mb: &mut T) -> bool {
    let cpu = mb.cpu_mut();
    if cpu.state.wait > 0 {
        cpu.state.wait -= 1;
        return false;
    }
    return true;
}

/// Unconditionally advance the state of the CPU
pub fn exec<T: WithCpu + BusDevice>(mb: &mut T) {
    let pc = mb.cpu().state.pc;
    let word = mb.read32(pc);
    let (mnemonic, instruction) = decode_instruction(word);
    let fn_handler = match_handler::<T>(mnemonic);
    fn_handler(mb, instruction);
    // update CPU state
    {
        let cpu = mb.cpu_mut();
        cpu.cycles += 1;
        cpu.state.pc += 4;
    }
}

//#region Cpu Instructions
#[allow(type_alias_bounds)] // leaving this in for self-documenting reasons
type OpcodeHandler<T: WithCpu + BusDevice> = fn(&mut T, Instruction);

fn match_handler<T: WithCpu + BusDevice>(mnemonic: Mnemonic) -> OpcodeHandler<T> {
    match mnemonic {
        Mnemonic::ADD => op_add,
        Mnemonic::ADDI => op_addi,
        Mnemonic::ADDIU => op_addiu,
        Mnemonic::ADDU => op_addu,
        Mnemonic::LUI => op_lui,
        Mnemonic::ORI => op_ori,
        Mnemonic::SW => op_sw,
        _ => panic!("Operation {:?} not implemented", mnemonic),
    }
}

op_fn!(op_add, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    cpu.state.registers[dest] =
        cpu.state.registers[source].wrapping_add(cpu.state.registers[target]);
    // TODO: overflow exception routing via COP0
});

op_fn!(op_addi, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    cpu.state.registers[target] = cpu.state.registers[source].wrapping_add(data);
    // TODO: overflow exception
});

op_fn!(op_addiu, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    cpu.state.registers[target] = cpu.state.registers[source].wrapping_add(data);
});

op_fn!(op_addu, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    cpu.state.registers[dest] =
        cpu.state.registers[source].wrapping_add(cpu.state.registers[target]);
});

// skip

op_fn!(op_lui, (mb, instr), {
    let data = u32::from(instr.immediate()) << 16;
    let cpu = mb.cpu_mut();
    cpu.state.registers[instr.rt() as usize] = data;
});

// skip

op_fn!(op_ori, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = zero_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    cpu.state.registers[target] = cpu.state.registers[source] | data;
});

// skip

op_fn!(op_sw, (mb, instr), {
    let base = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let addr = mb.cpu().state.registers[base] + data;
    mb.write32(addr, mb.cpu().state.registers[target]);
});

//#endregion

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constructs() {
        let cpu = CpuR3000::new();
        assert_eq!(
            cpu.state.pc, CPU_POWERON_STATE.pc,
            "Program counter is not at the reset vector"
        );
    }
}
