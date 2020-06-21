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
    /// The next instruction to execute in the pipeline, as a word
    ///
    /// This is implemented to simulate delay slots, which occur due to how the
    /// MIPS architecture handles (or more accurately, doesn't handle) branch
    /// hazards in instructions.
    next_instruction: u32,
}

pub const CPU_POWERON_STATE: CpuState = CpuState {
    // from IDX docs
    pc: 0xBFC0_0000,
    // the rest of this is shooting from the hip
    registers: [0u32; 32],
    next_instruction: 0x0000_00000,
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

fn write_reg(cpu: &mut CpuR3000, addr: usize, data: u32) {
    cpu.state.registers[addr] = data;
    cpu.state.registers[0] = 0; // coerce the 0-register to be 0
}

fn get_reg(cpu: &CpuR3000, addr: usize) -> u32 {
    return cpu.state.registers[addr];
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
    let (mnemonic, instruction) = decode_instruction(mb.cpu().state.next_instruction);
    mb.cpu_mut().state.next_instruction = mb.read32(pc);
    println!("STEP {:?} 0x{:08X}", mnemonic, *instruction);
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

#[rustfmt::skip]
fn match_handler<T: WithCpu + BusDevice>(mnemonic: Mnemonic) -> OpcodeHandler<T> {
    match mnemonic {
        Mnemonic::ADD => op_add,
        Mnemonic::ADDI => op_addi,
        Mnemonic::ADDIU => op_addiu,
        Mnemonic::ADDU => op_addu,
        Mnemonic::AND =>            /*op_and,*/todo!("instr {:?}", mnemonic),
        Mnemonic::ANDI =>           /*op_andi,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BEQ =>            /*op_beq,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BGEZ =>           /*op_bgez,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BGEZAL =>         /*op_bgezal,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BGTZ =>           /*op_bgtz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BLEZ =>           /*op_blez,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BLTZ =>           /*op_bltz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BLTZAL =>         /*op_bltzal,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BNE =>            /*op_bne,*/todo!("instr {:?}", mnemonic),
        Mnemonic::BREAK =>          /*op_break,*/todo!("instr {:?}", mnemonic),
        Mnemonic::CFCz =>           /*op_cfcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::COPz =>           /*op_copz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::CTCz =>           /*op_ctcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::DIV =>            /*op_div,*/todo!("instr {:?}", mnemonic),
        Mnemonic::DIVU =>           /*op_divu,*/todo!("instr {:?}", mnemonic),
        Mnemonic::J => op_j,
        Mnemonic::JAL =>            /*op_jal,*/todo!("instr {:?}", mnemonic),
        Mnemonic::JALR =>           /*op_jalr,*/todo!("instr {:?}", mnemonic),
        Mnemonic::JR =>             /*op_jr,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LB =>             /*op_lb,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LBU =>            /*op_lbu,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LH =>             /*op_lh,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LHU =>            /*op_lhu,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LUI => op_lui,
        Mnemonic::LW =>             /*op_lw,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LWCz =>           /*op_lwcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LWL =>            /*op_lwl,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LWR =>            /*op_lwr,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MFCz =>           /*op_mfcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MFHI =>           /*op_mfhi,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MFLO =>           /*op_mflo,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MTCz =>           /*op_mtcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MTHI =>           /*op_mthi,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MTLO =>           /*op_mtlo,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MULT =>           /*op_mult,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MULTU =>          /*op_multu,*/todo!("instr {:?}", mnemonic),
        Mnemonic::NOR =>            /*op_nor,*/todo!("instr {:?}", mnemonic),
        Mnemonic::OR => op_or,
        Mnemonic::ORI => op_ori,
        Mnemonic::SB =>             /*op_sb,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SH =>             /*op_sh,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SLL => op_sll,
        Mnemonic::SLLV =>           /*op_sllv,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SLT =>            /*op_slt,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SLTI =>           /*op_slti,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SLTIU =>          /*op_sltiu,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SLTU =>           /*op_sltu,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SRA =>            /*op_sra,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SRAV =>           /*op_srav,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SRL =>            /*op_srl,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SRLV =>           /*op_srlv,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SUB =>            /*op_sub,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SUBU =>           /*op_subu,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SW => op_sw,
        Mnemonic::SWCz =>           /*op_swcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SWL =>            /*op_swl,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SWR =>            /*op_swr,*/todo!("instr {:?}", mnemonic),
        Mnemonic::SYSCALL =>        /*op_syscall,*/todo!("instr {:?}", mnemonic),
        Mnemonic::XOR =>            /*op_xor,*/todo!("instr {:?}", mnemonic),
        Mnemonic::XORI =>           /*op_xori,*/todo!("instr {:?}", mnemonic),
    }
}

op_fn!(op_add, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    write_reg(
        cpu,
        dest,
        get_reg(cpu, source).wrapping_add(get_reg(cpu, target)),
    );
    // TODO: overflow exception routing via COP0
});

op_fn!(op_addi, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    write_reg(cpu, target, get_reg(cpu, source).wrapping_add(data));
    // TODO: overflow exception
});

op_fn!(op_addiu, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    write_reg(cpu, target, get_reg(cpu, source).wrapping_add(data));
});

op_fn!(op_addu, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    write_reg(
        cpu,
        dest,
        get_reg(cpu, source).wrapping_add(get_reg(cpu, target)),
    );
});

// skip

op_fn!(op_j, (mb, instr), {
    let target = instr.target() << 2;
    let new_pc = target | mb.cpu().state.pc & 0xF000_0000; // select the 4 MSBs of the old PC
    mb.cpu_mut().state.pc = new_pc - 4; // correct for the PC advance later
    mb.cpu_mut().state.wait += 1; //
});

// skip

op_fn!(op_lui, (mb, instr), {
    let data = u32::from(instr.immediate()) << 16;
    let cpu = mb.cpu_mut();
    write_reg(cpu, instr.rt() as usize, data);
});

// skip

op_fn!(op_or, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    write_reg(cpu, dest, get_reg(cpu, source) | get_reg(cpu, target));
});

op_fn!(op_ori, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = zero_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    write_reg(cpu, target, get_reg(cpu, source) | data);
});

// skip

op_fn!(op_sll, (mb, instr), {
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let shamt = instr.shamt();
    let cpu = mb.cpu_mut();
    write_reg(cpu, dest, get_reg(cpu, target).wrapping_shl(shamt as u32));
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
