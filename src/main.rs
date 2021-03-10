#![feature(assoc_char_funcs)]

use bitvec::prelude::*;
mod instructions;
mod program;
mod cartridge;
use program::Program;
use cartridge::read_file;

fn main() {
    let x: Vec<u8> = Vec::from([0, 0, 0, 0, 17, 0, 0, 0]);
    // println!("{}", program::combine_u8(0, 16));
    let mut y = Program::from_u8(x);
    y.verbose = true;
    y.execute();

    let z = 1u8;
    println!("{:?}", read_file("cpu_dummy_reads.nes"))
}
