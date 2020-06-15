/// A trait for a device that can be connected to the main bus
pub trait BusDevice {
    /// Read a 32-bit word from the device at a local address
    fn read32(&mut self, addr: u32) -> u32;
    /// Attempt to read a 32-bit word without modifying state
    ///
    /// This is not always for every device, as MMIO reads can sometimes require
    /// mutability. In these cases, this function should return None.
    fn peek32(&self, addr: u32) -> Option<u32>;
    /// Write a 32-bit word to the given local address
    fn write32(&mut self, addr: u32, data: u32);
}
