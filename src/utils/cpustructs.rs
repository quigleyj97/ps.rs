/// Structs, enums, and helpers for modeling CPU state
use std::ops::Deref;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RegisterIndex {
    /// 0 register
    ///
    /// This register is a bit special, in hardware it's actually hard-wired to
    /// always be zero. No idea why you'd need that, but there you go.
    R0 = 0,
    /// Assembler Reserved register, alias R1
    AT = 1,
    /// Subroutine Return Register 0, alias R2
    V0 = 2,
    /// Subroutine Return Register 2, alias R3
    V1 = 3,
    /// Subroutine Argument Register 0, alias R4
    A0 = 4,
    /// Subroutine Argument Register 1, alias R5
    A1 = 5,
    /// Subroutine Argument Register 2, alias R6
    A2 = 6,
    /// Subroutine Argument Register 3, alias R7
    A3 = 7,
    //#region Temporary Registers
    /// Temporary Register T0, alias R8
    T0 = 8,
    /// Temporary Register T1, alias R9
    T1 = 9,
    /// Temporary Register T2, alias R10
    T2 = 10,
    /// Temporary Register T3, alias R11
    T3 = 11,
    /// Temporary Register T4, alias R12
    T4 = 12,
    /// Temporary Register T5, alias R13
    T5 = 13,
    /// Temporary Register T6, alias R14
    T6 = 14,
    /// Temporary Register T7, alias R15
    T7 = 15,
    //#endregion
    //#region Static registers
    /// Static Register S0, alias R16
    S0 = 16,
    /// Static Register S1, alias R17
    S1 = 17,
    /// Static Register S2, alias R18
    S2 = 18,
    /// Static Register S3, alias R19
    S3 = 19,
    /// Static Register S4, alias R20
    S4 = 20,
    /// Static Register S5, alias R21
    S5 = 21,
    /// Static Register S6, alias R22
    S6 = 22,
    /// Static Register S7, alias R23
    S7 = 23,
    //#endregion
    //#region Yet more temporary registers (yes, they're discontinuous!)
    /// Temporary Register T8, alias R24
    T8 = 24,
    /// Temporary Register T9, alias R25
    T9 = 25,
    //#endregion
    //#region Kernel registers
    /// Kernel Register K0, alias R26
    K0 = 26,
    /// Kernel Register K1, alias R27
    K1 = 27,
    //#endregion
    /// Global pointer, alias R28
    GP = 28,
    /// Stack pointer, alias R29
    SP = 29,
    /// Frame pointer, alias R30, also a "9th" static variable
    FP = 30,
    /// Return address, alias R31
    RA = 31,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Mnemonic {
    /// Add
    ADD,
    /// Add Immediate
    ADDI,
    /// Add Immediate Unsigned
    ADDIU,
    /// Add Unsigned
    ADDU,
    /// Logical AND
    AND,
    /// Logical AND Immediate
    ANDI,
    /// Branch on Equal
    BEQ,
    /// Branch on >= 0
    BGEZ,
    /// Branch on >=0 and link
    BGEZAL, // what an inscrutable mnemonic
    /// Branch on > 0
    BGTZ,
    /// Branch on <= 0
    BLEZ,
    /// Branch on < 0
    BLTZ,
    /// Branch on <= 0 and link
    BLTZAL,
    /// Branch on !==
    BNE,
    /// Breakpoint
    BREAK,
    /// Move control from coprocessor
    CFCz,
    /// Coprocessor operation
    COPz,
    /// Move control to coprocessor
    CTCz,
    /// Divide
    DIV,
    /// Divide Unsigned
    DIVU,
    /// Jump
    J,
    /// Jump and link
    JAL,
    /// Jump and link (register)
    JALR,
    /// Jump (register)
    JR,
    /// Load byte
    LB,
    /// Load byte unsigned
    LBU,
    /// Load half-word
    LH,
    /// Load half-word unsigned
    LHU,
    /// Load upper immediate
    LUI,
    /// Load word
    LW,
    /// Load word to coprocessor
    LWCz,
    /// Load word left
    LWL,
    /// Load word right
    LWR,
    /// Move from Coprocessor
    MFCz,
    /// Move from HI
    MFHI,
    /// Move from LO
    MFLO,
    /// Move to Coprocessor
    MTCz,
    /// Move to HI
    MTHI,
    /// Move to LO
    MTLO,
    /// Multiply
    MULT,
    /// Multiply unsigned
    MULTU,
    /// Logical NOR
    NOR,
    /// Logical OR
    OR,
    /// Logical OR immediate
    ORI, // and the blind forest
    /// Store byte
    SB,
    /// Store halfword
    SH, //it
    /// Logical shift word left
    SLL,
    /// Logical shift word left variable
    SLLV,
    /// Set on <
    SLT,
    /// Set on < immediate
    SLTI,
    /// Set on < immediate unsigned
    SLTIU,
    /// Set on < unsigned
    SLTU,
    /// Arithmetic shift right
    SRA,
    /// Arithmetic shift right variable
    SRAV,
    /// Logical shift right
    SRL,
    /// Logical shift right variable
    SRLV,
    /// Subtract
    SUB,
    /// Subtract unsigned
    SUBU,
    /// Store word
    SW,
    /// Store word from coprocessor
    SWCz,
    /// Store word left
    SWL,
    /// Store word right
    SWR,
    /// Syscall
    SYSCALL,
    /// Logical XOR
    XOR,
    /// Logical XOR immediate
    XORI,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InstructionFormat {
    Immediate,
    Jump,
    Register,
}

const INSTR_PART_OP: u32 = 0xFC00_0000;
const INSTR_PART_RS: u32 = 0x03E0_0000;
const INSTR_PART_RT: u32 = 0x001F_0000;
const INSTR_PART_RD: u32 = 0x0000_F800;
const INSTR_PART_SHAMT: u32 = 0x0000_07E0;
const INSTR_PART_FUNCT: u32 = 0x0000_003F;
const INSTR_PART_IMMEDIATE: u32 = 0x0000_FFFF;
const INSTR_PART_TARGET: u32 = 0x03FF_FFFF;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Instruction(pub u32);

impl Deref for Instruction {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        let Instruction(op) = self;
        return op;
    }
}

impl Instruction {
    pub fn op(&self) -> u8 {
        ((**self & INSTR_PART_OP) >> 26) as u8
    }

    pub fn rs(&self) -> u8 {
        ((**self & INSTR_PART_RS) >> 21) as u8
    }

    pub fn rt(&self) -> u8 {
        ((**self & INSTR_PART_RT) >> 16) as u8
    }

    pub fn rd(&self) -> u8 {
        ((**self & INSTR_PART_RD) >> 11) as u8
    }

    pub fn shamt(&self) -> u8 {
        ((**self & INSTR_PART_SHAMT) >> 6) as u8
    }

    pub fn funct(&self) -> u8 {
        (**self & INSTR_PART_FUNCT) as u8
    }

    pub fn immediate(&self) -> u16 {
        (**self & INSTR_PART_IMMEDIATE) as u16
    }

    pub fn target(&self) -> u32 {
        (**self & INSTR_PART_TARGET) as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constructs_instruction() {
        let _data = Instruction(0);
        assert!(true); // expect no errors
    }

    #[test]
    fn derefs_correctly() {
        let data = Instruction(0xCAFE_BABE);
        assert_eq!(*data, 0xCAFE_BABE);
    }

    #[test]
    fn splits_segments_correctly() {
        let data = Instruction(0xA5A5_A5A5);
        // this is (in binary):
        // 10100101_10100101_10100101_10100101
        // so we expect the following:
        // ______ opcode                        = 0b101001
        //       __ ___ rs                      = 0b01101
        //             _____ rt                 = 0b00101
        //                   _____ rd           = 0b10100
        //                        ___ __ shamt  = 0b10110
        //                        funct ______  = 0b100101
        // target__ ________ ________ ________  = 0x01A5_A5A5
        //         immediate ________ ________  = 0x0000_A5A5
        assert_eq!(data.op(), 0b101001, "op mismatch");
        assert_eq!(data.rs(), 0b01101, "rs mismatch");
        assert_eq!(data.rt(), 0b00101, "rt mismatch");
        assert_eq!(data.rd(), 0b10100, "rd mismatch");
        assert_eq!(data.shamt(), 0b10110, "shamt mismatch");
        assert_eq!(data.funct(), 0b100101, "funct mismatch");
        assert_eq!(data.target(), 0x01A5_A5A5, "target mismatch");
        assert_eq!(data.immediate(), 0x0000_A5A5, "immediate mismatch");
    }
}
