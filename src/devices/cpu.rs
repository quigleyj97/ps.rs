use crate::devices::motherboard::Motherboard;

#[derive(Clone, Debug)]
pub struct CpuState {
    /// Assembler Reserved register, alias R1
    pub at: u32,
    /// Subroutine Return Register 0, alias R2
    pub v0: u32,
    /// Subroutine Return Register 2, alias R3
    pub v1: u32,
    /// Subroutine Argument Register 0, alias R4
    pub a0: u32,
    /// Subroutine Argument Register 1, alias R5
    pub a1: u32,
    /// Subroutine Argument Register 2, alias R6
    pub a2: u32,
    /// Subroutine Argument Register 3, alias R7
    pub a3: u32,
    //#region Temporary Registers
    pub t0: u32,
    pub t1: u32,
    pub t2: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    pub t7: u32,
    //#endregion
    //#region Static registers
    pub s0: u32,
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s5: u32,
    pub s6: u32,
    pub s7: u32,
    //#endregion
    //#region Yet more temporary registers (yes, they're discontinuous!)
    pub t8: u32,
    pub t9: u32,
    //#endregion
    //#region Kernel registers
    pub k0: u32,
    pub k1: u32,
    //#endregion
    /// Global pointer, alias R28
    pub gp: u32,
    /// Stack pointer, alias R29
    pub sp: u32,
    /// Frame pointer, alias R30, also a "9th" static variable
    pub fp: u32,
    /// Return address, alias R31
    pub ra: u32,
    /// The program counter register
    pub pc: u32,
}

pub const CPU_POWERON_STATE: CpuState = CpuState {
    // from IDX docs
    pc: 0xBFC0_0000,
    // the rest of this is shooting from the hip
    at: 0,
    v0: 0,
    v1: 0,
    a0: 0,
    a1: 0,
    a2: 0,
    a3: 0,
    t0: 0,
    t1: 0,
    t2: 0,
    t3: 0,
    t4: 0,
    t5: 0,
    t6: 0,
    t7: 0,
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
    s4: 0,
    s5: 0,
    s6: 0,
    s7: 0,
    t8: 0,
    t9: 0,
    k0: 0,
    k1: 0,
    gp: 0,
    sp: 0,
    fp: 0,
    ra: 0,
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

pub fn exec(mb: &mut Motherboard) {
    mb.cpu.cycles += 1;
    mb.read32(mb.cpu.state.pc);
}
