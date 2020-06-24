use std::convert::TryInto;

/// A trait for a device that can be connected to the main bus
pub trait BusDevice {
    /// Read a data point from the device at a local address
    fn read<T: SizedData>(&mut self, addr: u32) -> T;
    /// Attempt to read a data point without modifying state
    ///
    /// This is not always for every device, as MMIO reads can sometimes require
    /// mutability. In these cases, this function should return None.
    fn peek<T: SizedData>(&self, addr: u32) -> Option<T>;
    /// Write a data point to the given local address
    fn write<T: SizedData>(&mut self, addr: u32, data: T);
}

/// Trait representing an addressable datapoint in memory
pub trait SizedData: Eq + Ord + std::fmt::UpperHex {
    /// Returns the size of this data in bytes
    fn width() -> usize;

    /// Returns whether the given address is properly aligned for this size
    fn is_aligned(addr: u32) -> bool;

    /// Given a slice of LE bytes, return a 'reconsituted' version
    fn from_le_byteslice(bytes: &[u8]) -> Self;

    /// Given a slice, write this value into that slice as LE bytes
    fn to_le_byteslice(&self, bytes: &mut [u8]);

    /// Given a u32, return a DataType with any MSBs that don't fit truncated
    fn from_u32(data: u32) -> Self;
}

impl SizedData for u8 {
    fn width() -> usize {
        1
    }

    fn is_aligned(_addr: u32) -> bool {
        true
    }

    fn from_le_byteslice(bytes: &[u8]) -> Self {
        bytes[0]
    }

    fn to_le_byteslice(&self, bytes: &mut [u8]) {
        bytes[..1].clone_from_slice(&self.to_le_bytes());
    }

    fn from_u32(data: u32) -> Self {
        (data & 0xFF) as u8
    }
}

impl SizedData for u16 {
    fn width() -> usize {
        2
    }

    fn is_aligned(addr: u32) -> bool {
        addr & 0b1 == 0
    }

    fn from_le_byteslice(bytes: &[u8]) -> Self {
        return u16::from_le_bytes(bytes.try_into().unwrap());
    }

    fn to_le_byteslice(&self, bytes: &mut [u8]) {
        bytes[..2].clone_from_slice(&self.to_le_bytes());
    }

    fn from_u32(data: u32) -> Self {
        (data & 0xFFFF) as u16
    }
}

impl SizedData for u32 {
    fn width() -> usize {
        4
    }

    fn is_aligned(addr: u32) -> bool {
        addr & 0b11 == 0
    }

    fn from_le_byteslice(bytes: &[u8]) -> Self {
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_byteslice(&self, bytes: &mut [u8]) {
        bytes[..4].clone_from_slice(&self.to_le_bytes());
    }

    fn from_u32(data: u32) -> Self {
        data
    }
}
