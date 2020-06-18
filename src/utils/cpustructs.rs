/// Structs, enums, and helpers for modeling CPU state
use std::ops::Deref;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum InstructionFormat {
    Immediate,
    Jump,
    Register,
}

/// The 6-bit opcode specifiers
///
/// MIPS-I can use a combination of this and a "function field" to fully
/// identify some mnemonics
#[rustfmt::skip]
enum Opcode {
    SPECIAL = 0b000000,
    REGIMM  = 0b000001,
    ADDI    = 0b001000,
    ADDIU   = 0b001001,
    ANDI    = 0b001100,
    BEQ     = 0b000100,
    BGTZ    = 0b000111,
    BLEZ    = 0b000110,
    BNE     = 0b000101,
    // technically these are COPz but I've chosen to encode them separately
    COP0    = 0b010000,
    COP1    = 0b010001,
    COP2    = 0b010010,
    COP3    = 0b010011,
    J       = 0b000010,
    JAL     = 0b000011,
    LB      = 0b100000,
    LBU     = 0b100100,
    LH      = 0b100001,
    LHU     = 0b100101,
    LUI     = 0b001111,
    LW      = 0b100011,
    LWCOP0  = 0b110000,
    LWCOP1  = 0b110001,
    LWCOP2  = 0b110010,
    LWCOP3  = 0b110011,
    LWL     = 0b100010,
    LWR     = 0b100110,
    ORI     = 0b001101,
    SB      = 0b101000,
    SH      = 0b101001,
    SLTI    = 0b001010,
    SLTIU   = 0b001011,
    SW      = 0b101011,
    SWCOP0  = 0b111000,
    SWCOP1  = 0b111001,
    SWCOP2  = 0b111010,
    SWCOP3  = 0b111011,
    SWL     = 0b101010,
    SWR     = 0b101110,
    XORI    = 0b001110,
}

const INSTR_PART_OP: u32 = 0xFC00_0000;
const INSTR_PART_RS: u32 = 0x03E0_0000;
const INSTR_PART_RT: u32 = 0x001F_0000;
const INSTR_PART_RD: u32 = 0x0000_F800;
const INSTR_PART_SHAMT: u32 = 0x0000_07E0;
const INSTR_PART_FUNCT: u32 = 0x0000_001F;
const INSTR_PART_IMMEDIATE: u32 = 0x0000_FFFF;
const INSTR_PART_TARGET: u32 = 0x07FF_FFFF;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Instruction(u32);

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
        ((**self & INSTR_PART_SHAMT) >> 5) as u8
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
const OP_COPz: u8 = 0b0100;
const OP_LWCz: u8 = 0b1100;
const OP_SWCz: u8 = 0b1110;

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

const RZ_BGEZ: u8 = 0b00001;
const RZ_BGEZAL: u8 = 0b10001;
const RZ_BLTZ: u8 = 0b00000;
const RZ_BLTZAL: u8 = 0b10000;

// todo: decode COP instructions
pub fn decode_instruction(word: u32) -> (Mnemonic, Instruction) {
    let instr = Instruction(word);
    let mnemonic = match instr.op() {
        OP_SPECIAL => decode_register_instruction(instr),
        OP_REGIMM => decode_regimm_instruction(instr),
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
        _ => panic!("Unexpected illegal opcode: 0x{:08X}", word),
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
