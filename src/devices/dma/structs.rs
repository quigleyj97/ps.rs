use std::ops::Deref;

//#region DMA channels
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DmaPort {
    MdecIn = 0,
    MdecOut = 1,
    Gpu = 2,
    CdRom = 3,
    Spu = 4,
    Pio = 5,
    Otc = 6,
}

impl From<usize> for DmaPort {
    fn from(op: usize) -> Self {
        match op {
            0 => DmaPort::MdecIn,
            1 => DmaPort::MdecOut,
            2 => DmaPort::Gpu,
            3 => DmaPort::CdRom,
            4 => DmaPort::Spu,
            5 => DmaPort::Pio,
            6 => DmaPort::Otc,
            _ => panic!("Not a valid DMA port: {}", op),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DmaChannelDirection {
    /// DMA will copy data _from_ main memory _to_ the device
    RamToDevice,
    /// DMA will copy data _from_ the device _to_ main memory
    DeviceToRam,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DmaChannelIteration {
    /// The channel increments the base address with each step
    Forward,
    /// The channel decrements the base address with each step
    Backward,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DmaChannelSync {
    /// The channel will begin copying as soon as it is enabled
    Manual,
    /// The channel will wait for a ready signal from the device
    Request,
    /// The channel will use a linked list to sync (GPU only)
    LinkedList,
}

// quite a few bits are unused, these should be ANDed out when deref-ing
const DMA_CHANNEL_UNUSED: u32 = 0x8E88_F8FC;
const DMA_CHANNEL_TRANSFER: u32 = 0x0000_0001;
const DMA_CHANNEL_INCREMENT: u32 = 0x0000_0002;
const DMA_CHANNEL_CHOPPING: u32 = 0x0000_0004;
const DMA_CHANNEL_SYNC_TYPE: u32 = 0x0000_0100;
const DMA_CHANNEL_CHOP_DMA_WINDOW: u32 = 0x0000_0600;
const DMA_CHANNEL_CHOP_CPU_WINDOW: u32 = 0x0007_0000;
const DMA_CHANNEL_ENABLE: u32 = 0x0100_0000;
const DMA_CHANNEL_MANUAL_TRIGGER: u32 = 0x1000_0000;
const DMA_CHANNEL_UNKNOWN: u32 = 0x6000_0000;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct DmaChannel(u32);

impl DmaChannel {
    pub fn get_direction(&self) -> DmaChannelDirection {
        return match **self & DMA_CHANNEL_TRANSFER {
            0 => DmaChannelDirection::RamToDevice,
            1 => DmaChannelDirection::DeviceToRam,
            _ => unreachable!(),
        };
    }

    pub fn get_iter_dir(&self) -> DmaChannelIteration {
        return match (**self & DMA_CHANNEL_INCREMENT) >> 1 {
            0 => DmaChannelIteration::Forward,
            1 => DmaChannelIteration::Backward,
            _ => unreachable!(),
        };
    }

    pub fn is_chop_enabled(&self) -> bool {
        return ((**self & DMA_CHANNEL_CHOPPING) >> 2) == 1;
    }

    pub fn get_sync_type(&self) -> DmaChannelSync {
        return match (**self & DMA_CHANNEL_SYNC_TYPE) >> 9 {
            0 => DmaChannelSync::Manual,
            1 => DmaChannelSync::Request,
            2 => DmaChannelSync::LinkedList,
            // I have no idea what actual hardware does in this case
            3 => panic!("DMA controller attempting to use reserved sync mode"),
            _ => unreachable!(),
        };
    }

    pub fn get_dma_chop_window(&self) -> u8 {
        return (0xFF & ((**self & DMA_CHANNEL_CHOP_DMA_WINDOW) >> 16)) as u8;
    }

    pub fn get_cpu_chop_window(&self) -> u8 {
        return (0xFF & ((**self & DMA_CHANNEL_CHOP_CPU_WINDOW) >> 20)) as u8;
    }

    pub fn is_enabled(&self) -> bool {
        return ((**self & DMA_CHANNEL_ENABLE) >> 24) == 1;
    }

    pub fn is_manually_triggered(&self) -> bool {
        return ((**self & DMA_CHANNEL_MANUAL_TRIGGER) >> 28) == 1;
    }

    pub fn get_unknown_bits(&self) -> u8 {
        return (0xFF & ((**self * DMA_CHANNEL_UNKNOWN) >> 29)) as u8;
    }
}

impl From<u32> for DmaChannel {
    fn from(data: u32) -> Self {
        DmaChannel(data & !DMA_CHANNEL_UNUSED)
    }
}

impl Deref for DmaChannel {
    type Target = u32;

    fn deref(&self) -> &u32 {
        return &self.0;
    }
}
//#endregion
