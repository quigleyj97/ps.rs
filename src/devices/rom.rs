use crate::devices::bus::BusDevice;

/// A read-only region of memory
pub struct Rom {
    buf: Vec<u8>,
}

impl Rom {
    pub fn from_buf(buf: Vec<u8>) -> Rom {
        return Rom { buf };
    }

    fn read(&self, addr: usize) -> u32 {
        let b0 = self.buf[addr + 0] as u32;
        let b1 = self.buf[addr + 1] as u32;
        let b2 = self.buf[addr + 2] as u32;
        let b3 = self.buf[addr + 3] as u32;

        return b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
    }
}

impl BusDevice for Rom {
    fn peek32(&self, addr: u32) -> Option<u32> {
        // cast addr to usize, then read from buffer
        return Some(self.read(addr as usize));
    }

    fn read32(&mut self, addr: u32) -> u32 {
        return self.read(addr as usize);
    }

    fn write32(&mut self, _addr: u32, _data: u32) {
        // no-op
    }
}
