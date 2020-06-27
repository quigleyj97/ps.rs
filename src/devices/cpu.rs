use crate::devices::bus::{BusDevice, SizedData};
use crate::devices::cop0::Cop0;
use crate::utils::cpustructs::{CpuState, Exception, Instruction, Mnemonic, CPU_POWERON_STATE};
use crate::utils::decode::decode_instruction;
use crate::utils::disasm::disasm_instr;
use log::{debug, trace};

macro_rules! sign_extend {
    ($val: expr) => {{
        (($val as i16) as u32)
    }};
}

macro_rules! zero_extend {
    ($val: expr) => {{
        (($val as u16) as u32)
    }};
}

macro_rules! op_fn {
    ($mnemonic:ident, ($cpu: ident, $instr: ident), $body: expr) => {
        fn $mnemonic<T: WithCpu + BusDevice>(
            $cpu: &mut T,
            $instr: Instruction,
        ) -> Option<Exception> {
            $body
        }
    };
}

/// The CPU for the PlayStation
///
/// This CPU is a MIPS ISA with a 5-stage pipeline
pub struct CpuR3000 {
    state: CpuState,
    pub cycles: u64,
    pub cop0: Cop0,
}

impl CpuR3000 {
    pub fn new() -> CpuR3000 {
        return CpuR3000 {
            state: CPU_POWERON_STATE.clone(),
            cycles: 0,
            cop0: Cop0::new(),
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
    cpu.state.registers[0] = 0;
}

fn get_reg(cpu: &CpuR3000, addr: usize) -> u32 {
    return cpu.state.registers[addr];
}

fn branch(cpu: &mut CpuR3000, offset: u16) {
    let new_pc = cpu
        .state
        .pc
        .wrapping_add(sign_extend!((offset as u32) << 2));
    cpu.state.pc = new_pc - 4; // correct for PC advance
}

fn read<T: WithCpu + BusDevice, D: SizedData>(mb: &mut T, addr: u32) -> D {
    return mb.read::<D>(addr);
}

fn write<T: WithCpu + BusDevice, D: SizedData>(mb: &mut T, addr: u32, data: D) {
    if mb.cpu().cop0.is_cache_isolated() {
        debug!(target: "cpu", "Cache isolation active, but cache is unimplemented");
        return;
    }
    return mb.write(addr, data);
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
    let (cur_instruction, cur_pc) = mb.cpu().state.next_instruction;
    let next_pc = mb.cpu().state.pc;
    // pre-execution updates
    {
        let next_instruction = mb.read::<u32>(next_pc);
        let cpu = mb.cpu_mut();
        // advance the PC
        cpu.state.next_instruction = (next_instruction, next_pc);
        // execute any pipelined loads
        let (reg_idx, val) = cpu.state.next_load;
        cpu.state.registers[reg_idx] = val;
        cpu.state.next_load = (0, 0);
    }

    let (mnemonic, instruction) = decode_instruction(cur_instruction);
    trace!(target: "cpu", "STEP ${:08X} 0x{:08X} SP={:08X} RA={:08X} {}", cur_pc, *instruction, mb.cpu().state.registers[29],mb.cpu().state.registers[31], disasm_instr(mnemonic, instruction));
    let fn_handler = match_handler::<T>(mnemonic);
    match fn_handler(mb, instruction) {
        None => {} // do nothing- operation completed successfully
        Some(e) => {
            // normally we'd route this to cop0 to handle, but I haven't
            // implemented much of that coprocessor yet.
            todo!("Exception handling via cop0 for exception {:?}", e);
        }
    }
    // post-execution updates
    {
        let cpu = mb.cpu_mut();
        cpu.cycles += 1;
        cpu.state.pc += 4;
    }
}

//#region Cpu Instructions
#[allow(type_alias_bounds)] // leaving this in for self-documenting reasons
type OpcodeHandler<T: WithCpu + BusDevice> = fn(&mut T, Instruction) -> Option<Exception>;

#[rustfmt::skip]
fn match_handler<T: WithCpu + BusDevice>(mnemonic: Mnemonic) -> OpcodeHandler<T> {
    match mnemonic {
        Mnemonic::ADD => op_add,
        Mnemonic::ADDI => op_addi,
        Mnemonic::ADDIU => op_addiu,
        Mnemonic::ADDU => op_addu,
        Mnemonic::AND => op_and,
        Mnemonic::ANDI => op_andi,
        Mnemonic::BEQ => op_beq,
        Mnemonic::BGEZ => op_bgez,
        Mnemonic::BGEZAL => op_bgezal,
        Mnemonic::BGTZ => op_bgtz,
        Mnemonic::BLEZ => op_blez,
        Mnemonic::BLTZ => op_bltz,
        Mnemonic::BLTZAL => op_bltzal,
        Mnemonic::BNE => op_bne,
        Mnemonic::BREAK =>          /*op_break,*/todo!("instr {:?}", mnemonic),
        Mnemonic::CFCz =>           /*op_cfcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::COPz =>           /*op_copz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::CTCz =>           /*op_ctcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::DIV => op_div,
        Mnemonic::DIVU => op_divu,
        Mnemonic::J => op_j,
        Mnemonic::JAL => op_jal,
        Mnemonic::JALR => op_jalr,
        Mnemonic::JR => op_jr,
        Mnemonic::LB => op_lb,
        Mnemonic::LBU => op_lbu,
        Mnemonic::LH => op_lh,
        Mnemonic::LHU => op_lhu,
        Mnemonic::LUI => op_lui,
        Mnemonic::LW => op_lw,
        Mnemonic::LWCz =>           /*op_lwcz,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LWL =>            /*op_lwl,*/todo!("instr {:?}", mnemonic),
        Mnemonic::LWR =>            /*op_lwr,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MFCz => op_mfcz,
        Mnemonic::MFHI => op_mfhi,
        Mnemonic::MFLO => op_mflo,
        Mnemonic::MTCz => op_mtcz,
        Mnemonic::MTHI =>           /*op_mthi,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MTLO =>           /*op_mtlo,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MULT =>           /*op_mult,*/todo!("instr {:?}", mnemonic),
        Mnemonic::MULTU =>          /*op_multu,*/todo!("instr {:?}", mnemonic),
        Mnemonic::NOR =>            /*op_nor,*/todo!("instr {:?}", mnemonic),
        Mnemonic::OR => op_or,
        Mnemonic::ORI => op_ori,
        Mnemonic::SB => op_sb,
        Mnemonic::SH => op_sh,
        Mnemonic::SLL => op_sll,
        Mnemonic::SLLV => op_sllv,
        Mnemonic::SLT => op_slt,
        Mnemonic::SLTI => op_slti,
        Mnemonic::SLTIU => op_sltiu,
        Mnemonic::SLTU => op_sltu,
        Mnemonic::SRA => op_sra,
        Mnemonic::SRAV => op_srav,
        Mnemonic::SRL => op_srl,
        Mnemonic::SRLV => op_srlv,
        Mnemonic::SUB => op_sub,
        Mnemonic::SUBU => op_subu,
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
    let source_data = get_reg(cpu, source);
    let target_data = get_reg(cpu, target);
    // test for overflow
    match (source_data as i32).checked_add(target_data as i32) {
        Some(res) => {
            write_reg(cpu, dest, res as u32);
            None
        }
        None => Some(Exception::IntegerOverflow),
    }
});

op_fn!(op_addi, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    let source_data = get_reg(cpu, source);
    match (source_data as i32).checked_add(data as i32) {
        Some(res) => {
            write_reg(cpu, target, res as u32);
            None
        }
        None => Some(Exception::IntegerOverflow),
    }
});

op_fn!(op_addiu, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    write_reg(cpu, target, get_reg(cpu, source).wrapping_add(data));
    None
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
    None
});

op_fn!(op_and, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    write_reg(cpu, dest, get_reg(cpu, source) & get_reg(cpu, target));
    None
});

op_fn!(op_andi, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = zero_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    write_reg(cpu, target, get_reg(cpu, source) & data);
    None
});

op_fn!(op_beq, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    if get_reg(mb.cpu(), source) == get_reg(mb.cpu(), target) {
        branch(mb.cpu_mut(), instr.immediate());
    }
    None
});

op_fn!(op_bgez, (mb, instr), {
    let source = instr.rs() as usize;
    if (get_reg(mb.cpu(), source) as i32) >= 0 {
        branch(mb.cpu_mut(), instr.immediate());
    }
    None
});

op_fn!(op_bgezal, (mb, instr), {
    let source = instr.rs() as usize;
    let pc = mb.cpu().state.pc;
    write_reg(mb.cpu_mut(), 31, pc);
    if (get_reg(mb.cpu(), source) as i32) >= 0 {
        branch(mb.cpu_mut(), instr.immediate());
    }
    None
});

op_fn!(op_bgtz, (mb, instr), {
    let source = instr.rs() as usize;
    if (get_reg(mb.cpu(), source) as i32) > 0 {
        branch(mb.cpu_mut(), instr.immediate());
    }
    None
});

op_fn!(op_blez, (mb, instr), {
    let source = instr.rs() as usize;
    if (get_reg(mb.cpu(), source) as i32) <= 0 {
        branch(mb.cpu_mut(), instr.immediate());
    }
    None
});

op_fn!(op_bltz, (mb, instr), {
    let source = instr.rs() as usize;
    if (get_reg(mb.cpu(), source) as i32) < 0 {
        branch(mb.cpu_mut(), instr.immediate());
    }
    None
});

op_fn!(op_bltzal, (mb, instr), {
    let source = instr.rs() as usize;
    let pc = mb.cpu().state.pc;
    write_reg(mb.cpu_mut(), 31, pc);
    if (get_reg(mb.cpu(), source) as i32) < 0 {
        branch(mb.cpu_mut(), instr.immediate());
    }
    None
});

op_fn!(op_bne, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    if get_reg(mb.cpu(), source) != get_reg(mb.cpu(), target) {
        branch(mb.cpu_mut(), instr.immediate());
    }
    None
});

// skip

op_fn!(op_div, (mb, instr), {
    let cpu = mb.cpu_mut();
    let numerator = get_reg(cpu, instr.rs() as usize) as i32;
    let denominator = get_reg(cpu, instr.rt() as usize) as i32;

    // divide-by-zeros actually don't result in exceptions, instead the CPU just
    // puts garbage into the HI and LO registers
    if denominator == 0 {
        cpu.state.hi = numerator as u32;
        cpu.state.lo = if numerator >= 0 {
            0xFFFF_FFFF
        } else {
            0x0000_0001
        };
        return None;
    }

    // additionally, attempting to divide i32::MIN (-2mil something) by -1
    // results in a number that is too large to store in a 32-bit int. So the
    // CPU also inserts garbage into the result registers
    if numerator == i32::MIN && denominator == -1 {
        cpu.state.hi = 0;
        cpu.state.lo = i32::MIN as u32;
        return None;
    }

    // finally do the happy-path integer division
    cpu.state.hi = (numerator % denominator) as u32;
    cpu.state.lo = (numerator / denominator) as u32;
    None
});

op_fn!(op_divu, (mb, instr), {
    let cpu = mb.cpu_mut();
    let numerator = get_reg(cpu, instr.rs() as usize);
    let denominator = get_reg(cpu, instr.rt() as usize);

    // divide-by-zeros actually don't result in exceptions, instead the CPU just
    // puts garbage into the HI and LO registers
    if denominator == 0 {
        cpu.state.hi = numerator;
        cpu.state.lo = if (numerator as i32) >= 0 {
            0xFFFF_FFFF
        } else {
            0x0000_0001
        };
        return None;
    }

    // DIVU doesn't have the same caveats as DIV with i32::MIN, go directly to
    // happy-path integer division
    cpu.state.hi = numerator % denominator;
    cpu.state.lo = numerator / denominator;
    None
});

op_fn!(op_j, (mb, instr), {
    let target = instr.target() << 2;
    let new_pc = target | mb.cpu().state.pc & 0xF000_0000; // select the 4 MSBs of the old PC
    mb.cpu_mut().state.pc = new_pc - 4; // correct for the PC advance later
    None
});

op_fn!(op_jal, (mb, instr), {
    // 31 = RA register
    let pc = mb.cpu().state.pc;
    write_reg(mb.cpu_mut(), 31, pc);
    // re-use the J op
    op_j(mb, instr)
});

op_fn!(op_jalr, (mb, instr), {
    // 31 = RA register
    let pc = mb.cpu().state.pc;
    write_reg(mb.cpu_mut(), 31, pc);
    let jmp_to = get_reg(mb.cpu(), instr.rs() as usize);
    mb.cpu_mut().state.pc = jmp_to;
    None
});

op_fn!(op_jr, (mb, instr), {
    let jmp_to = get_reg(mb.cpu(), instr.rs() as usize);
    mb.cpu_mut().state.pc = jmp_to;
    None
});

op_fn!(op_lb, (mb, instr), {
    let base = get_reg(mb.cpu(), instr.rs() as usize);
    let addr = base.wrapping_add(sign_extend!(instr.immediate()));
    // todo: read errors
    let data = read::<T, u8>(mb, addr) as i8;

    mb.cpu_mut().state.next_load = (instr.rt() as usize, data as u32);
    None
});

op_fn!(op_lbu, (mb, instr), {
    let base = get_reg(mb.cpu(), instr.rs() as usize);
    let addr = base.wrapping_add(sign_extend!(instr.immediate()));
    // todo: read errors
    let data = read::<T, u8>(mb, addr) as u8;

    mb.cpu_mut().state.next_load = (instr.rt() as usize, data as u32);
    None
});

op_fn!(op_lh, (mb, instr), {
    let base = get_reg(mb.cpu(), instr.rs() as usize);
    let addr = base.wrapping_add(sign_extend!(instr.immediate()));
    // todo: read errors
    let data = read::<T, u8>(mb, addr) as i16;

    mb.cpu_mut().state.next_load = (instr.rt() as usize, data as u32);
    None
});

op_fn!(op_lhu, (mb, instr), {
    let base = get_reg(mb.cpu(), instr.rs() as usize);
    let addr = base.wrapping_add(sign_extend!(instr.immediate()));
    // todo: read errors
    let data = read::<T, u8>(mb, addr) as u16;

    mb.cpu_mut().state.next_load = (instr.rt() as usize, data as u32);
    None
});

op_fn!(op_lui, (mb, instr), {
    let data = u32::from(instr.immediate()) << 16;
    let cpu = mb.cpu_mut();
    write_reg(cpu, instr.rt() as usize, data);
    None
});

op_fn!(op_lw, (mb, instr), {
    let base = get_reg(mb.cpu(), instr.rs() as usize);
    let addr = base.wrapping_add(sign_extend!(instr.immediate()));
    // todo: read errors

    let data = read(mb, addr);

    mb.cpu_mut().state.next_load = (instr.rt() as usize, data);

    None
});

// skip

op_fn!(op_mfcz, (mb, instr), {
    let coproc = instr.op() & 0b11;
    match coproc {
        0 => {
            let data = mb.cpu_mut().cop0.mfc(instr.rd() as usize);
            mb.cpu_mut().state.next_load = (instr.rt() as usize, data);
            None
        }
        // TODO: Cop2 is the GTE
        _ => Some(Exception::CoprocessorUnusable),
    }
});

op_fn!(op_mfhi, (mb, instr), {
    let reg = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    write_reg(cpu, reg, cpu.state.hi);
    None
});

op_fn!(op_mflo, (mb, instr), {
    let reg = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    write_reg(cpu, reg, cpu.state.lo);
    None
});

op_fn!(op_mtcz, (mb, instr), {
    let coproc = instr.op() & 0b11;
    let data = get_reg(mb.cpu(), instr.rt() as usize);
    match coproc {
        0 => {
            mb.cpu_mut().cop0.mtc(instr.rd() as usize, data);
            None
        }
        // TODO: Cop2 is the GTE
        _ => Some(Exception::CoprocessorUnusable),
    }
});

// skip

op_fn!(op_or, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    write_reg(cpu, dest, get_reg(cpu, source) | get_reg(cpu, target));
    None
});

op_fn!(op_ori, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = zero_extend!(instr.immediate());
    let cpu = mb.cpu_mut();
    write_reg(cpu, target, get_reg(cpu, source) | data);
    None
});

op_fn!(op_sb, (mb, instr), {
    let base = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let addr = mb.cpu().state.registers[base].wrapping_add(data);
    write(mb, addr, (get_reg(mb.cpu(), target) & 0xFF) as u8);
    // todo: addr, bus, TLB exceptions
    None
});

op_fn!(op_sh, (mb, instr), {
    let base = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let addr = mb.cpu().state.registers[base].wrapping_add(data);
    write(mb, addr, (get_reg(mb.cpu(), target) & 0xFFFF) as u16);
    // todo: addr, bus, TLB exceptions
    None
});

op_fn!(op_sll, (mb, instr), {
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let shamt = instr.shamt();
    let cpu = mb.cpu_mut();
    write_reg(cpu, dest, get_reg(cpu, target).wrapping_shl(shamt as u32));
    None
});

op_fn!(op_sllv, (mb, instr), {
    let target = get_reg(mb.cpu(), instr.rt() as usize);
    let dest = instr.rd() as usize;
    let shift = get_reg(mb.cpu(), instr.rs() as usize) & 0b0001_1111;
    write_reg(mb.cpu_mut(), dest, target.wrapping_shl(shift as u32));
    None
});

op_fn!(op_slt, (mb, instr), {
    let target = get_reg(mb.cpu(), instr.rt() as usize);
    let source = get_reg(mb.cpu(), instr.rs() as usize);
    write_reg(
        mb.cpu_mut(),
        instr.rd() as usize,
        if (source as i32) < (target as i32) {
            1
        } else {
            0
        },
    );
    None
});

op_fn!(op_slti, (mb, instr), {
    let target = sign_extend!(instr.immediate());
    let source = get_reg(mb.cpu(), instr.rs() as usize);
    write_reg(
        mb.cpu_mut(),
        instr.rt() as usize,
        if (source as i32) < (target as i32) {
            1
        } else {
            0
        },
    );
    None
});

op_fn!(op_sltiu, (mb, instr), {
    let target = sign_extend!(instr.immediate());
    let source = get_reg(mb.cpu(), instr.rs() as usize);
    write_reg(
        mb.cpu_mut(),
        instr.rt() as usize,
        if source < target { 1 } else { 0 },
    );
    None
});

op_fn!(op_sltu, (mb, instr), {
    let target = get_reg(mb.cpu(), instr.rt() as usize);
    let source = get_reg(mb.cpu(), instr.rs() as usize);
    write_reg(
        mb.cpu_mut(),
        instr.rd() as usize,
        if source < target { 1 } else { 0 },
    );
    None
});

op_fn!(op_sra, (mb, instr), {
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let shamt = instr.shamt();
    let cpu = mb.cpu_mut();
    write_reg(
        cpu,
        dest,
        (get_reg(cpu, target) as i32).wrapping_shr(shamt as u32) as u32,
    );
    None
});

op_fn!(op_srav, (mb, instr), {
    let target = get_reg(mb.cpu(), instr.rt() as usize) as i32;
    let dest = instr.rd() as usize;
    let shift = get_reg(mb.cpu(), instr.rs() as usize) & 0b0001_1111;
    write_reg(mb.cpu_mut(), dest, target.wrapping_shr(shift as u32) as u32);
    None
});

op_fn!(op_srl, (mb, instr), {
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let shamt = instr.shamt();
    let cpu = mb.cpu_mut();
    write_reg(cpu, dest, get_reg(cpu, target).wrapping_shr(shamt as u32));
    None
});

op_fn!(op_srlv, (mb, instr), {
    let target = get_reg(mb.cpu(), instr.rt() as usize);
    let dest = instr.rd() as usize;
    let shift = get_reg(mb.cpu(), instr.rs() as usize) & 0b0001_1111;
    write_reg(mb.cpu_mut(), dest, target.wrapping_shr(shift as u32));
    None
});

op_fn!(op_sub, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    let source_data = get_reg(cpu, source);
    let target_data = get_reg(cpu, target);
    // test for overflow
    match (source_data as i32).checked_sub(target_data as i32) {
        Some(res) => {
            write_reg(cpu, dest, res as u32);
            None
        }
        None => Some(Exception::IntegerOverflow),
    }
});

op_fn!(op_subu, (mb, instr), {
    let source = instr.rs() as usize;
    let target = instr.rt() as usize;
    let dest = instr.rd() as usize;
    let cpu = mb.cpu_mut();
    let source_data = get_reg(cpu, source);
    let target_data = get_reg(cpu, target);
    // test for overflow
    let res = (source_data as i32).wrapping_sub(target_data as i32);
    write_reg(cpu, dest, res as u32);
    None
});

op_fn!(op_sw, (mb, instr), {
    let base = instr.rs() as usize;
    let target = instr.rt() as usize;
    let data = sign_extend!(instr.immediate());
    let addr = mb.cpu().state.registers[base].wrapping_add(data);
    // TODO: TLB refill/invalid/modified exceptions
    // TODO: Bus errors
    // TODO: Address errors
    write(mb, addr, get_reg(mb.cpu(), target));
    None
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
