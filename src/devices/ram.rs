use super::bus::{BusDevice, SizedData};

pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn with_size(size: usize) -> Ram {
        return Ram {
            data: vec![0u8; size],
        };
    }

    fn read_buf<T: SizedData>(&self, addr: usize) -> T {
        return T::from_le_byteslice(&self.data[addr..(addr + T::width())]);
    }

    fn write_buf<T: SizedData>(&mut self, addr: usize, data: T) {
        data.to_le_byteslice(&mut self.data[addr..(addr + T::width())])
    }
}

impl BusDevice for Ram {
    fn read<T: SizedData>(&mut self, addr: u32) -> T {
        self.read_buf::<T>(addr as usize)
    }

    fn peek<T: SizedData>(&self, addr: u32) -> Option<T> {
        Some(self.read_buf::<T>(addr as usize))
    }
    fn write<T: SizedData>(&mut self, addr: u32, data: T) {
        self.write_buf(addr as usize, data);
    }
}
