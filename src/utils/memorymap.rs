/// The segments of memory that the CPU can address
#[derive(Debug, Eq, PartialEq)]
pub enum Segment {
    KUSEG,
    KSEG0,
    KSEG1,
    KSEG2,
}

/// The devices on the main bus
#[derive(Debug, Eq, PartialEq)]
pub enum Device {
    /// The system RAM
    RAM,
    /// The first expansion area
    Expansion1,
    /// Scratchpad/fast data cache
    Scratch,
    /// IO ports
    IO,
    /// The second expansion area
    Expansion2,
    /// The third expansion area
    Expansion3,
    /// The internal BIOS stored on a 512kib ROM
    BIOS,
    /// The IO and cache control ports
    IOCacheControl,
    /// No device exists at this address
    None,
    /// Address lies outside of virtual memory (KUSEG only)
    VMemException,
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    start: u32,
    end: u32,
    length: u32,
}

impl Range {
    const fn new(start: u32, length: u32) -> Range {
        let end = start.wrapping_add(length);
        Range { start, length, end }
    }
}

//#region KSEG consts
const KUSEG_RANGE: Range = Range::new(0x0000_0000, 0x8000_0000);
const KSEG0_RANGE: Range = Range::new(0x8000_0000, 0x2000_0000);
const KSEG1_RANGE: Range = Range::new(0xA000_0000, 0x2000_0000);
const KSEG2_RANGE: Range = Range::new(0xC000_0000, 0x4000_0000);
//#endregion

//#region Device map consts

// These values come from No$Psx, so the device descriptions may not be correct.
// If that's the case I'll rename the devices in the enums (if I remember to do
// that).

const RAM_RANGE: Range = Range::new(0x0000_0000, 2048 * 1024);
const EXP1_RANGE: Range = Range::new(0x0F00_0000, 8192 * 1024);
const SCRATCH_RANGE: Range = Range::new(0x0F80_0000, 1024);
const IO_RANGE: Range = Range::new(0x0F80_1000, 2 * 1024); // deviating from no$psx here
const EXP2_RANGE: Range = Range::new(0x0F80_2000, 8 * 1024);
const EXP3_RANGE: Range = Range::new(0x0FA0_0000, 2048 * 1024);
const BIOS_RANGE: Range = Range::new(0x0FC0_0000, 512 * 1024);
const CACHE_CTRL_RANGE: Range = Range::new(0x3FFE_0000, 512);
//#endregion

/// Given an address, return a 3-tuple of the memory segment, the device, and
/// the device-local address.
pub fn map_device(addr: u32) -> (Segment, Device, u32) {
    let segment = if addr < KUSEG_RANGE.end {
        Segment::KUSEG
    } else if addr < KSEG0_RANGE.end {
        Segment::KSEG0
    } else if addr < KSEG1_RANGE.end {
        Segment::KSEG1
    } else {
        Segment::KSEG2
    };
    if segment == Segment::KSEG2 {
        let addr = addr - KSEG2_RANGE.start;
        if addr < CACHE_CTRL_RANGE.start || addr >= CACHE_CTRL_RANGE.end {
            panic!("Invalid KSEG2 address: ${:08X}", addr + KSEG2_RANGE.start);
        }
        return (
            segment,
            Device::IOCacheControl,
            addr - CACHE_CTRL_RANGE.start,
        );
    }
    // KUSEG, KSEG0, and KSEG1 are mirrors of each other in the PSX
    let seg_local_addr = addr & 0x0FFF_FFFF;
    if seg_local_addr > 0x2000_0000 && segment == Segment::KUSEG {
        // address is larger than the memory map, throw a CPU exception
        return (segment, Device::VMemException, seg_local_addr);
    }
    let (device, local_addr) =
        if seg_local_addr >= RAM_RANGE.start && seg_local_addr < RAM_RANGE.end {
            (Device::RAM, seg_local_addr - RAM_RANGE.start)
        } else if seg_local_addr >= EXP1_RANGE.start && seg_local_addr < EXP1_RANGE.end {
            (Device::Expansion1, seg_local_addr - EXP1_RANGE.start)
        } else if seg_local_addr >= SCRATCH_RANGE.start
            && seg_local_addr < SCRATCH_RANGE.end
            && segment != Segment::KSEG1
        {
            (Device::Scratch, seg_local_addr - SCRATCH_RANGE.start)
        } else if seg_local_addr >= IO_RANGE.start && seg_local_addr < IO_RANGE.end {
            (Device::IO, seg_local_addr - IO_RANGE.start)
        } else if seg_local_addr >= EXP2_RANGE.start && seg_local_addr < EXP2_RANGE.end {
            (Device::Expansion2, seg_local_addr - EXP2_RANGE.start)
        } else if seg_local_addr >= EXP3_RANGE.start && seg_local_addr < EXP3_RANGE.end {
            (Device::Expansion3, seg_local_addr - EXP3_RANGE.start)
        } else if seg_local_addr >= BIOS_RANGE.start && seg_local_addr < BIOS_RANGE.end {
            (Device::BIOS, seg_local_addr - BIOS_RANGE.start)
        } else {
            panic!(
                "Invalid memory location in {:?}: ${:08X} / ${:08X}",
                segment, addr, seg_local_addr
            )
        };
    return (segment, device, local_addr);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn maps_to_bios() {
        const BIOS_RESET_ADDR: u32 = 0xBFC0_0000;
        assert_eq!(
            map_device(BIOS_RESET_ADDR),
            (Segment::KSEG1, Device::BIOS, 0)
        );
        assert_eq!(
            map_device(BIOS_RESET_ADDR + 1),
            (Segment::KSEG1, Device::BIOS, 1)
        );
    }

    #[test]
    fn maps_segments() {
        const BIOS_RESET_ADDR: u32 = 0x1FC0_0000;
        assert_eq!(
            map_device(BIOS_RESET_ADDR),
            (Segment::KUSEG, Device::BIOS, 0)
        );
        assert_eq!(
            map_device(BIOS_RESET_ADDR + KSEG0_RANGE.start),
            (Segment::KSEG0, Device::BIOS, 0)
        );
        assert_eq!(
            map_device(BIOS_RESET_ADDR + KSEG1_RANGE.start),
            (Segment::KSEG1, Device::BIOS, 0)
        );
        assert_eq!(
            map_device(0xFFFE_0000),
            (Segment::KSEG2, Device::IOCacheControl, 0)
        );
    }

    #[test]
    fn maps_expansion_regions() {
        assert_eq!(
            map_device(0x1F00_0000),
            (Segment::KUSEG, Device::Expansion1, 0)
        );
        assert_eq!(
            map_device(0x1F80_2000),
            (Segment::KUSEG, Device::Expansion2, 0)
        );
        assert_eq!(
            map_device(0x1FA0_0000),
            (Segment::KUSEG, Device::Expansion3, 0)
        );
    }

    #[test]
    fn maps_scratchpad_in_cached_segments() {
        assert_eq!(
            map_device(0x9F80_0000),
            (Segment::KSEG0, Device::Scratch, 0)
        );
    }

    #[test]
    #[should_panic(expected = "Invalid memory location in KSEG1: 0xBF800000")]
    fn panics_when_attempting_to_map_scratchpad_to_kseg1() {
        map_device(0xBF80_0000);
    }
}
