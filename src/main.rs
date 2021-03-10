#![feature(assoc_char_funcs)]

mod instructions;
mod program;
use program::Program;

fn main() {
    let x: Vec<u8> = Vec::from([0, 0, 0, 0, 17, 0, 0, 0]);
    // println!("{}", program::combine_u8(0, 16));
    let mut y = Program::from_u8(x);
    y.verbose = true;
    y.execute();
}
