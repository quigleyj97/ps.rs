# ps.rs

A PSX emulator written in Rust

![Build](https://github.com/quigleyj97/ps.rs/workflows/Rust/badge.svg)

## Project goals

1. Cross-compile to WebAssembly targets (dependent on WebGPU)
2. Play a sampling of NTSC-US games correctly
3. Work with open-source/emulated BIOSes to simplify distribution

## Building

Run `cargo build` to build the project. Tests can be run with `cargo test`.

## Running

The emulator requires a BIOS from a PSX, which can be dumped from physical
hardware or found online. The bios should have the following hash:

    sha1: 10155d8d6e6e832d6ea66db9bc098321fb5e8ebf
    md5: 924e392ed05558ffdb115408c263dccf
    crc32: 37157331

Place this in a project-root 'bios' folder, and name it `SCPH1001.BIN`. If your
filesystem is case-sensitive, use all upper-case letters.

Then, run the emulator with `cargo run`.

## Resources

 - Flandrin, Lionel. _Playstation Emulation Guide_ (version a89043e), 2016. https://github.com/simias/psx-guide.
 - Korth, Matrin. _Nocash PSX Specifications_, August 12, 2017. https://problemkaputt.de/psx-spx.htm.
 - Sweetman, Dominic, and Nigel Stephens. _IDT 30XX Family Software Reference Manual_. 1.0 rev. Integrated Device Technology, 1994. http://hitmen.c02.at/files/docs/psx/3467.pdf.
 - Walker, Joshua. _Everything You Have Always Wanted to Know about the Playstation But Were Afraid to Ask_ (version 1.1), 2000. http://hitmen.c02.at/files/docs/psx/psx.pdf.

