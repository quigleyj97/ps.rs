use crate::devices::bus::BusDevice;
use log::debug;

/// A trait for devices that own a GPU, such as the Motherboard
pub trait WithGpu {
    fn gpu(&self) -> &Gpu;
    fn gpu_mut(&mut self) -> &mut Gpu;
}

/// The 32-bit Toshiba custom GPU used on the PSX
///
/// For now, this is just a mock
pub struct Gpu {}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {}
    }
}

impl BusDevice for Gpu {
    fn read<T: crate::devices::bus::SizedData>(&mut self, addr: u32) -> T {
        debug!(target: "gpu", "Read from GP{}", addr / 4);
        // mock the DMAREADY flag
        T::from_u32(0x1000_0000)
    }
    fn peek<T: crate::devices::bus::SizedData>(&self, addr: u32) -> Option<T> {
        debug!(target: "gpu", "Peek from GP{}", addr / 4);
        Some(T::from_u32(0))
    }
    fn write<T: crate::devices::bus::SizedData>(&mut self, addr: u32, data: T) {
        debug!(target: "gpu", "Write to GP{} = 0x{:08X}", addr / 4, data);
    }
}
