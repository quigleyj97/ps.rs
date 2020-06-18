use crate::devices::motherboard::Motherboard;

#[derive(Clone, Debug)]
pub struct CpuState {
    /// The CPU registers
    registers: [u32; 32],
    /// The program counter register
    pub pc: u32,
}

pub const CPU_POWERON_STATE: CpuState = CpuState {
    // from IDX docs
    pc: 0xBFC0_0000,
    // the rest of this is shooting from the hip
    registers: [0u32; 32],
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
