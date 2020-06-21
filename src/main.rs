extern crate log;
extern crate pretty_env_logger;

pub mod devices;
pub mod utils;

use crate::devices::motherboard::Motherboard;
use log::info;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

fn main() {
    pretty_env_logger::init();

    let bios = read_bios().expect("Could not read BIOS");

    let mut psx = Motherboard::new(bios);

    info!(target: "main", "Starting emulation...");

    loop {
        psx.tick();
    }
}

fn read_bios() -> Result<Vec<u8>> {
    const BIOS_PATH: &str = "./bios/SCPH1001.bin";
    info!(target: "main", "Loading bios from pwd: {:?}", BIOS_PATH);

    let path = Path::new(&BIOS_PATH);
    let mut file =
        File::open(path).expect("BIOS not found in working directory: ./bios/SCPH1001.bin");
    let mut buf = vec![0u8; 524_288]; // 524,288 = number of bytes in 512kib

    file.read_exact(&mut buf[..])?;

    info!(target: "main", "BIOS loaded");

    Result::Ok(buf)
}
