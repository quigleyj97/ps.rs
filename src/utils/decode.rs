use crate::devices::cpu::structs::{Instruction, Mnemonic};
use log::debug;

//#region opcode consts
const OP_SPECIAL: u8 = 0b000000;
const OP_REGIMM: u8 = 0b000001;
const OP_ADDI: u8 = 0b001000;
const OP_ADDIU: u8 = 0b001001;
const OP_ANDI: u8 = 0b001100;
const OP_BEQ: u8 = 0b000100;
const OP_BGTZ: u8 = 0b000111;
const OP_BLEZ: u8 = 0b000110;
const OP_BNE: u8 = 0b000101;
const OP_J: u8 = 0b000010;
const OP_JAL: u8 = 0b000011;
const OP_LB: u8 = 0b100000;
const OP_LBU: u8 = 0b100100;
const OP_LH: u8 = 0b100001;
const OP_LHU: u8 = 0b100101;
const OP_LUI: u8 = 0b001111;
const OP_LW: u8 = 0b100011;
const OP_LWL: u8 = 0b100010;
const OP_LWR: u8 = 0b100110;
const OP_ORI: u8 = 0b001101;
const OP_SB: u8 = 0b101000;
const OP_SH: u8 = 0b101001;
const OP_SLTI: u8 = 0b001010;
const OP_SLTIU: u8 = 0b001011;
const OP_SW: u8 = 0b101011;
const OP_SWL: u8 = 0b101010;
const OP_SWR: u8 = 0b101110;
const OP_XORI: u8 = 0b001110;
// COPz instructions
#[allow(non_upper_case_globals)]
const OP_COPz: u8 = 0b0100;
#[allow(non_upper_case_globals)]
const OP_LWCz: u8 = 0b1100;
#[allow(non_upper_case_globals)]
const OP_SWCz: u8 = 0b1110;
//#endregion

//#region function consts
const FUNCT_ADD: u8 = 0b100000;
const FUNCT_ADDU: u8 = 0b100001;
const FUNCT_AND: u8 = 0b100100;
const FUNCT_BREAK: u8 = 0b001101;
const FUNCT_DIV: u8 = 0b011010;
const FUNCT_DIVU: u8 = 0b011011;
const FUNCT_JALR: u8 = 0b001001;
const FUNCT_JR: u8 = 0b001000;
const FUNCT_MFHI: u8 = 0b010000;
const FUNCT_MFLO: u8 = 0b010010;
const FUNCT_MTHI: u8 = 0b010001;
const FUNCT_MTLO: u8 = 0b010011;
const FUNCT_MULT: u8 = 0b011000;
const FUNCT_MULTU: u8 = 0b011001;
const FUNCT_NOR: u8 = 0b100111;
const FUNCT_OR: u8 = 0b100101;
const FUNCT_SLL: u8 = 0b000000;
const FUNCT_SLLV: u8 = 0b000100;
const FUNCT_SLT: u8 = 0b101010;
const FUNCT_SLTU: u8 = 0b101011;
const FUNCT_SRA: u8 = 0b000011;
const FUNCT_SRAV: u8 = 0b000111;
const FUNCT_SRL: u8 = 0b000010;
const FUNCT_SRLV: u8 = 0b000110;
const FUNCT_SUB: u8 = 0b100010;
const FUNCT_SUBU: u8 = 0b100011;
const FUNCT_SYSCALL: u8 = 0b001100;
const FUNCT_XOR: u8 = 0b100110;
//#endregion

//#region RZ consts
const RZ_BGEZ: u8 = 0b00001;
const RZ_BGEZAL: u8 = 0b10001;
const RZ_BLTZ: u8 = 0b00000;
const RZ_BLTZAL: u8 = 0b10000;
//#endregion

/// Decode a MIPS-I instruction, returning a 2-tuple of the instruction
/// mneominic and a struct to access data fields.
pub fn decode_instruction(word: u32) -> (Mnemonic, Instruction) {
    let instr = Instruction(word);
    let mnemonic = match instr.op() {
        OP_SPECIAL => decode_register_instruction(instr),
        OP_REGIMM => decode_regimm_instruction(instr),
        op if (op >> 2) == OP_COPz => decode_copz_instruction(instr),
        op if (op >> 2) == OP_LWCz => Mnemonic::LWCz,
        op if (op >> 2) == OP_SWCz => Mnemonic::SWCz,
        OP_ADDI => Mnemonic::ADDI,
        OP_ADDIU => Mnemonic::ADDIU,
        OP_ANDI => Mnemonic::ANDI,
        OP_BEQ => Mnemonic::BEQ,
        OP_BGTZ => Mnemonic::BGTZ,
        OP_BLEZ => Mnemonic::BLEZ,
        OP_BNE => Mnemonic::BNE,
        OP_J => Mnemonic::J,
        OP_JAL => Mnemonic::JAL,
        OP_LB => Mnemonic::LB,
        OP_LBU => Mnemonic::LBU,
        OP_LH => Mnemonic::LH,
        OP_LHU => Mnemonic::LHU,
        OP_LUI => Mnemonic::LUI,
        OP_LW => Mnemonic::LW,
        OP_LWL => Mnemonic::LWL,
        OP_LWR => Mnemonic::LWR,
        OP_ORI => Mnemonic::ORI,
        OP_SB => Mnemonic::SB,
        OP_SH => Mnemonic::SH,
        OP_SLTI => Mnemonic::SLTI,
        OP_SLTIU => Mnemonic::SLTIU,
        OP_SW => Mnemonic::SW,
        OP_SWL => Mnemonic::SWL,
        OP_SWR => Mnemonic::SWR,
        OP_XORI => Mnemonic::XORI,
        _ => {
            debug!(target: "cpudec", "Illegal opcode encountered: 0x{:08X}", word);
            Mnemonic::__ILLEGAL__
        }
    };
    return (mnemonic, instr);
}

fn decode_register_instruction(instr: Instruction) -> Mnemonic {
    match instr.funct() {
        FUNCT_ADD => Mnemonic::ADD,
        FUNCT_ADDU => Mnemonic::ADDU,
        FUNCT_AND => Mnemonic::AND,
        FUNCT_BREAK => Mnemonic::BREAK,
        FUNCT_DIV => Mnemonic::DIV,
        FUNCT_DIVU => Mnemonic::DIVU,
        FUNCT_JALR => Mnemonic::JALR,
        FUNCT_JR => Mnemonic::JR,
        FUNCT_MFHI => Mnemonic::MFHI,
        FUNCT_MFLO => Mnemonic::MFLO,
        FUNCT_MTHI => Mnemonic::MTHI,
        FUNCT_MTLO => Mnemonic::MTLO,
        FUNCT_MULT => Mnemonic::MULT,
        FUNCT_MULTU => Mnemonic::MULTU,
        FUNCT_NOR => Mnemonic::NOR,
        FUNCT_OR => Mnemonic::OR,
        FUNCT_SLL => Mnemonic::SLL,
        FUNCT_SLLV => Mnemonic::SLLV,
        FUNCT_SLT => Mnemonic::SLT,
        FUNCT_SLTU => Mnemonic::SLTU,
        FUNCT_SRA => Mnemonic::SRA,
        FUNCT_SRAV => Mnemonic::SRAV,
        FUNCT_SRL => Mnemonic::SRL,
        FUNCT_SRLV => Mnemonic::SRLV,
        FUNCT_SUB => Mnemonic::SUB,
        FUNCT_SUBU => Mnemonic::SUBU,
        FUNCT_SYSCALL => Mnemonic::SYSCALL,
        FUNCT_XOR => Mnemonic::XOR,
        _ => panic!("Illegal funct: 0b{:05b} / 0x{:08X}", instr.funct(), *instr),
    }
}

fn decode_regimm_instruction(instr: Instruction) -> Mnemonic {
    match instr.rt() {
        RZ_BGEZ => Mnemonic::BGEZ,
        RZ_BGEZAL => Mnemonic::BGEZAL,
        RZ_BLTZ => Mnemonic::BLTZ,
        RZ_BLTZAL => Mnemonic::BLTZAL,
        _ => panic!("Illegal rt: 0b{:05b} / 0x{:08X}", instr.rt(), *instr),
    }
}

fn decode_copz_instruction(instr: Instruction) -> Mnemonic {
    return match instr.rs() {
        0b10000 => Mnemonic::COPz,
        0b00010 => Mnemonic::CFCz,
        0b00000 => Mnemonic::MFCz,
        0b00100 => Mnemonic::MTCz,
        _ => panic!("Invalid COPz instruction: 0x{:08X}", *instr),
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decodes_instr() {
        const ANDI_INSTR: u32 = 0x3000_0000;
        let (mnemonic, _instr) = decode_instruction(ANDI_INSTR);
        assert_eq!(mnemonic, Mnemonic::ANDI);
    }

    #[test]
    fn decodes_funct_instr() {
        const BREAK_INSTR: u32 = 0x0000_000D;
        let (mnemonic, _instr) = decode_instruction(BREAK_INSTR);
        assert_eq!(mnemonic, Mnemonic::BREAK);
    }

    #[test]
    fn decodes_cop_instr() {
        const COP0_INSTR: u32 = 0b0100001 << 25;
        let (mnemonic, _instr) = decode_instruction(COP0_INSTR);
        assert_eq!(mnemonic, Mnemonic::COPz);
    }

    #[test]
    fn decodes_rz_instr() {
        const BLTZ_INSTR: u32 = 0x0400_0000;
        let (mnemonic, _instr) = decode_instruction(BLTZ_INSTR);
        assert_eq!(mnemonic, Mnemonic::BLTZ);
    }

    #[test]
    fn decodes_mtc0_instr() {
        const MTC0_INSTR: u32 = 0x408C_6000;
        let (mnemonic, _instr) = decode_instruction(MTC0_INSTR);
        assert_eq!(mnemonic, Mnemonic::MTCz);
    }
}
