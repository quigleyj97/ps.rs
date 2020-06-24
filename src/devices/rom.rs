use crate::devices::bus::{BusDevice, SizedData};

/// A read-only region of memory
pub struct Rom {
    buf: Vec<u8>,
}

impl Rom {
    pub fn from_buf(buf: Vec<u8>) -> Rom {
        return Rom { buf };
    }

    fn read_buf<T: SizedData>(&self, addr: usize) -> T {
        return T::from_le_byteslice(&self.buf[addr..(addr + T::width())]);
    }
}

impl BusDevice for Rom {
    fn peek<T: SizedData>(&self, addr: u32) -> Option<T> {
        // cast addr to usize, then read from buffer
        return Some(self.read_buf::<T>(addr as usize));
    }

    fn read<T: SizedData>(&mut self, addr: u32) -> T {
        return self.read_buf::<T>(addr as usize);
    }

    fn write<T: SizedData>(&mut self, _addr: u32, _data: T) {
        // no-op
    }
}
