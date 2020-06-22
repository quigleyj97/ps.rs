use super::bus::BusDevice;

pub struct Ram {
    data: Vec<u32>,
}

impl Ram {
    pub fn with_size(size: usize) -> Ram {
        return Ram {
            data: vec![0u32; size],
        };
    }
}

impl BusDevice for Ram {
    fn read32(&mut self, addr: u32) -> u32 {
        // the bus uses aligned reads
        return self.data[(addr >> 2) as usize];
    }
    fn peek32(&self, addr: u32) -> Option<u32> {
        return Some(self.data[(addr >> 2) as usize]);
    }
    fn write32(&mut self, addr: u32, data: u32) {
        self.data[(addr >> 2) as usize] = data;
    }
}
