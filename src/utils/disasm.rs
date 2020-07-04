//! Utilities for pretty-printing instructions

use super::cpustructs::{CpuState, Instruction, Mnemonic};

/// Given an instruction mnemonic, return a MIPS asm representation
///
/// Note that register addresses are given as numbers instead of names, and
/// this will not attempt to translate pseudo-mnemonics (like NOP, which is
/// really a SLL $0, $0, 0- which conveniently encodes to 0x0000_0000)
///
/// For debugging purposes, you may also wish to log the state of registers
/// referenced by instructions- use pprint_instr for this purpose
pub fn disasm_instr(mnemonic: Mnemonic, instr: Instruction) -> String {
    match mnemonic {
        Mnemonic::ADD => disasm_r_instr(mnemonic, instr),
        Mnemonic::ADDI => disasm_i_instr(mnemonic, instr),
        Mnemonic::ADDIU => disasm_i_instr(mnemonic, instr),
        Mnemonic::ADDU => disasm_r_instr(mnemonic, instr),
        Mnemonic::AND => disasm_r_instr(mnemonic, instr),
        Mnemonic::ANDI => disasm_i_instr(mnemonic, instr),
        Mnemonic::BEQ => format!(
            "{:?} ${}, ${}, 0x{:04X}",
            mnemonic,
            instr.rs(),
            instr.rt(),
            instr.immediate()
        ),
        Mnemonic::BGEZ => disasm_branch_instr(mnemonic, instr),
        Mnemonic::BGEZAL => disasm_branch_instr(mnemonic, instr),
        Mnemonic::BGTZ => disasm_branch_instr(mnemonic, instr),
        Mnemonic::BLEZ => disasm_branch_instr(mnemonic, instr),
        Mnemonic::BLTZ => disasm_branch_instr(mnemonic, instr),
        Mnemonic::BLTZAL => disasm_branch_instr(mnemonic, instr),
        Mnemonic::BNE => format!(
            "{:?} ${}, ${}, 0x{:04X}",
            mnemonic,
            instr.rs(),
            instr.rt(),
            instr.immediate()
        ),
        Mnemonic::BREAK => disasm_bare_instr(mnemonic),
        Mnemonic::CFCz => format!("CFC{} ${}, ${}", instr.op() & 0b11, instr.rt(), instr.rd()),
        Mnemonic::COPz => format!("COP{} 0x{:08X}", instr.op() & 0b11, *instr & 0x01FF_FFFF),
        Mnemonic::CTCz => format!("CTC{} ${}, ${}", instr.op() & 0b11, instr.rt(), instr.rd()),
        Mnemonic::DIV => disasm_math_instr(mnemonic, instr),
        Mnemonic::DIVU => disasm_math_instr(mnemonic, instr),
        Mnemonic::J => disasm_j_instr(mnemonic, instr),
        Mnemonic::JAL => disasm_j_instr(mnemonic, instr),
        Mnemonic::JALR => format!("{:?} ${}, ${}", mnemonic, instr.rd(), instr.rs()),
        Mnemonic::JR => format!("{:?} ${}", mnemonic, instr.rs()),
        Mnemonic::LB => disasm_bus_instr(mnemonic, instr),
        Mnemonic::LBU => disasm_bus_instr(mnemonic, instr),
        Mnemonic::LH => disasm_bus_instr(mnemonic, instr),
        Mnemonic::LHU => disasm_bus_instr(mnemonic, instr),
        Mnemonic::LUI => format!(
            "{:?} ${}, 0x{:04X}",
            mnemonic,
            instr.rt(),
            instr.immediate()
        ),
        Mnemonic::LW => disasm_bus_instr(mnemonic, instr),
        Mnemonic::LWCz => format!(
            "LWC{} ${}, 0x{:04X}(${})",
            instr.op() & 0b11,
            instr.rt(),
            instr.immediate(),
            instr.rs()
        ),
        Mnemonic::LWL => disasm_bus_instr(mnemonic, instr),
        Mnemonic::LWR => disasm_bus_instr(mnemonic, instr),
        Mnemonic::MFCz => format!("MFC{} ${}, ${}", instr.op() & 0b11, instr.rt(), instr.rd()),
        Mnemonic::MFHI => format!("{:?} ${}", mnemonic, instr.rd()),
        Mnemonic::MFLO => format!("{:?} ${}", mnemonic, instr.rd()),
        Mnemonic::MTCz => format!("MTC{} ${}, ${}", instr.op() & 0b11, instr.rt(), instr.rd()),
        Mnemonic::MTHI => format!("{:?} ${}", mnemonic, instr.rs()),
        Mnemonic::MTLO => format!("{:?} ${}", mnemonic, instr.rs()),
        Mnemonic::MULT => disasm_math_instr(mnemonic, instr),
        Mnemonic::MULTU => disasm_math_instr(mnemonic, instr),
        Mnemonic::NOR => disasm_r_instr(mnemonic, instr),
        Mnemonic::OR => disasm_r_instr(mnemonic, instr),
        Mnemonic::ORI => disasm_i_instr(mnemonic, instr),
        Mnemonic::SB => disasm_bus_instr(mnemonic, instr),
        Mnemonic::SH => disasm_bus_instr(mnemonic, instr),
        Mnemonic::SLL => format!(
            "{:?} ${}, ${}, {}",
            mnemonic,
            instr.rd(),
            instr.rt(),
            instr.shamt()
        ),
        Mnemonic::SLLV => disasm_r_instr(mnemonic, instr),
        Mnemonic::SLT => disasm_r_instr(mnemonic, instr),
        Mnemonic::SLTI => disasm_i_instr(mnemonic, instr),
        Mnemonic::SLTIU => disasm_i_instr(mnemonic, instr),
        Mnemonic::SLTU => disasm_r_instr(mnemonic, instr),
        Mnemonic::SRA => format!(
            "{:?} ${}, ${}, {}",
            mnemonic,
            instr.rd(),
            instr.rt(),
            instr.shamt()
        ),
        Mnemonic::SRAV => disasm_r_instr(mnemonic, instr),
        Mnemonic::SRL => format!(
            "{:?} ${}, ${}, {}",
            mnemonic,
            instr.rd(),
            instr.rt(),
            instr.shamt()
        ),
        Mnemonic::SRLV => disasm_r_instr(mnemonic, instr),
        Mnemonic::SUB => disasm_r_instr(mnemonic, instr),
        Mnemonic::SUBU => disasm_r_instr(mnemonic, instr),
        Mnemonic::SW => disasm_bus_instr(mnemonic, instr),
        Mnemonic::SWCz => format!(
            "SWC{} ${}, 0x{:04X}(${})",
            instr.op() & 0b11,
            instr.rt(),
            instr.immediate(),
            instr.rs()
        ),
        Mnemonic::SWL => disasm_bus_instr(mnemonic, instr),
        Mnemonic::SWR => disasm_bus_instr(mnemonic, instr),
        Mnemonic::SYSCALL => disasm_bare_instr(mnemonic),
        Mnemonic::XOR => disasm_r_instr(mnemonic, instr),
        Mnemonic::XORI => disasm_i_instr(mnemonic, instr),
    }
}

pub fn pprint_instr(mnemonic: Mnemonic, instr: Instruction, state: &CpuState) -> String {
    let state_str = match get_referenced_registers(mnemonic, instr) {
        ReferencedRegisters::None => String::from(""),
        ReferencedRegisters::One(reg) => format!("${}=0x{:08X}", reg, state.registers[reg]),
        ReferencedRegisters::Two(reg1, reg2) => format!(
            "${}=0x{:08X}, ${}=0x{:08X}",
            reg1, state.registers[reg1], reg2, state.registers[reg2]
        ),
    };
    format!("{:30}; {}", disasm_instr(mnemonic, instr), state_str)
}

fn disasm_i_instr(mnemonic: Mnemonic, instr: Instruction) -> String {
    format!(
        "{:?} ${}, ${}, {}",
        mnemonic,
        instr.rt(),
        instr.rs(),
        instr.immediate() as i16
    )
}

fn disasm_j_instr(mnemonic: Mnemonic, instr: Instruction) -> String {
    // try to be helpful and give a real address
    format!("{:?} $_{:07X}", mnemonic, instr.target() << 2)
}

fn disasm_r_instr(mnemonic: Mnemonic, instr: Instruction) -> String {
    format!(
        "{:?} ${}, ${}, ${}",
        mnemonic,
        instr.rd(),
        instr.rs(),
        instr.rt()
    )
}

fn disasm_branch_instr(mnemonic: Mnemonic, instr: Instruction) -> String {
    format!(
        "{:?} ${}, {}",
        mnemonic,
        instr.rs(),
        instr.immediate() as i16
    )
}

fn disasm_bare_instr(mnemonic: Mnemonic) -> String {
    format!("{:?}", mnemonic)
}

fn disasm_bus_instr(mnemonic: Mnemonic, instr: Instruction) -> String {
    format!(
        "{:?} ${}, {}(${})",
        mnemonic,
        instr.rt(),
        instr.immediate() as i16,
        instr.rs()
    )
}

// for some reason DIV and MULT use this format instead
fn disasm_math_instr(mnemonic: Mnemonic, instr: Instruction) -> String {
    format!("{:?} ${}, ${}", mnemonic, instr.rs(), instr.rt())
}

enum ReferencedRegisters {
    None,
    One(usize),
    Two(usize, usize),
}

fn get_referenced_registers(mnemonic: Mnemonic, instr: Instruction) -> ReferencedRegisters {
    match mnemonic {
        Mnemonic::ADD => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::ADDI => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::ADDIU => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::ADDU => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::AND => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::ANDI => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::BEQ => ReferencedRegisters::Two(instr.rs() as usize, instr.rt() as usize),
        Mnemonic::BGEZ => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::BGEZAL => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::BGTZ => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::BLEZ => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::BLTZ => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::BLTZAL => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::BNE => ReferencedRegisters::Two(instr.rs() as usize, instr.rt() as usize),
        Mnemonic::BREAK => ReferencedRegisters::None,
        Mnemonic::CFCz => ReferencedRegisters::None,
        Mnemonic::COPz => ReferencedRegisters::None,
        Mnemonic::CTCz => ReferencedRegisters::None,
        Mnemonic::DIV => ReferencedRegisters::Two(instr.rs() as usize, instr.rt() as usize),
        Mnemonic::DIVU => ReferencedRegisters::Two(instr.rs() as usize, instr.rt() as usize),
        Mnemonic::J => ReferencedRegisters::None,
        Mnemonic::JAL => ReferencedRegisters::None,
        Mnemonic::JALR => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::JR => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::LB => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::LBU => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::LH => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::LHU => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::LUI => ReferencedRegisters::None,
        Mnemonic::LW => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::LWCz => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::LWL => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::LWR => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::MFCz => ReferencedRegisters::None,
        Mnemonic::MFHI => ReferencedRegisters::One(instr.rd() as usize),
        Mnemonic::MFLO => ReferencedRegisters::One(instr.rd() as usize),
        Mnemonic::MTCz => ReferencedRegisters::None,
        Mnemonic::MTHI => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::MTLO => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::MULT => ReferencedRegisters::Two(instr.rs() as usize, instr.rt() as usize),
        Mnemonic::MULTU => ReferencedRegisters::Two(instr.rs() as usize, instr.rt() as usize),
        Mnemonic::NOR => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::OR => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::ORI => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::SB => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::SH => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::SLL => ReferencedRegisters::One(instr.rt() as usize),
        Mnemonic::SLLV => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::SLT => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::SLTI => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::SLTIU => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::SLTU => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::SRA => ReferencedRegisters::One(instr.rt() as usize),
        Mnemonic::SRAV => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::SRL => ReferencedRegisters::One(instr.rt() as usize),
        Mnemonic::SRLV => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::SUB => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::SUBU => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::SW => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::SWCz => ReferencedRegisters::None,
        Mnemonic::SWL => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::SWR => ReferencedRegisters::One(instr.rs() as usize),
        Mnemonic::SYSCALL => ReferencedRegisters::None,
        Mnemonic::XOR => ReferencedRegisters::Two(instr.rt() as usize, instr.rs() as usize),
        Mnemonic::XORI => ReferencedRegisters::One(instr.rs() as usize),
    }
}
