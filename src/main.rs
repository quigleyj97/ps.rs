pub mod devices;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

use crate::devices::motherboard::Motherboard;

fn main() {
    println!("Hello, world!");

    let bios = read_bios().expect("Could not read BIOS");

    let psx = Motherboard::new(bios);

    println!("Read bios!");
}

fn read_bios() -> Result<Vec<u8>> {
    const BIOS_PATH: &str = "./bios/SCPH1001.bin";

    let path = Path::new(&BIOS_PATH);
    let mut file =
        File::open(path).expect("BIOS not found in working directory: ./bios/SCPH1001.bin");
    let mut buf = vec![0u8; 524_288]; // 524,288 = number of bytes in 512kib

    file.read_exact(&mut buf[..])?;

    Result::Ok(buf)
}
