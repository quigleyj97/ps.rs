mod cop0;
mod cpu;

pub use self::cpu::{exec, tick, CpuR3000, WithCpu};
pub mod structs;
