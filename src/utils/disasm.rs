//! Utilities for pretty-printing instructions

use super::cpustructs::{Instruction, Mnemonic};

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
        Mnemonic::MTHI => format!("{:?} ${}", mnemonic, instr.rd()),
        Mnemonic::MTLO => format!("{:?} ${}", mnemonic, instr.rd()),
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
    format!("{:?} $_{:7X}", mnemonic, instr.target() << 2)
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
