//! More information at http://wiki.nesdev.com/w/index.php/INES

use bitvec::prelude::*;
use std::{fs, error::Error};

#[derive(Debug)]
enum MirroringMode {
    Vertical,
    Horizontal
}

#[derive(Debug)]
struct Flags6 {
    mirroring: MirroringMode,
    battery: bool,
    trainer: bool,
    ignore_mirroring: bool,
}

#[derive(Debug)]
struct CartridgeHeader {
    prg_size: u8,
    chr_size: Option<u8>,
    mapper: u8,
    flag6: Flags6
}

#[derive(Debug)]
pub struct Cartridge {
    header: CartridgeHeader
}

fn to_u8(bits: &[u8]) -> u8 {
    bits.iter()
        .fold(0, |result, &bit| {
            (result << 1) ^ bit
        })
}

fn check_header(file: &Vec<u8>) -> Result<(), &str> {
    let header = file.get(0..4);
    match header {
        Some([78, 69, 83, 26]) => Ok(()),
        _ => Err("File does not contain valid NES header")
    }
    
}

pub fn read_file(path: &str) -> Result<Cartridge, Box<dyn Error + 'static>> {
    let file = fs::read(path)?;
    check_header(&file)?;

    let f6 = BitArray::<Lsb0, _>::new(file[6]);
    let f7 = BitArray::<Lsb0, _>::new(file[7]);

    let mapper_lower = f6.get(4..8).unwrap().iter().map(|b| *b as u8).collect::<Vec<u8>>();
    let mut mapper_upper = f7.get(4..8).unwrap().iter().map(|b| *b as u8).collect::<Vec<u8>>();
    mapper_upper.extend(mapper_lower);

    let mapper = to_u8(&mapper_upper);

    Ok(Cartridge {
        header: CartridgeHeader {
            prg_size: file[4],
            chr_size: match file.get(5) {
                Some(0) | None => None,
                _ => Some(file[5])
            },
            mapper,
            flag6: Flags6 {
                mirroring: match f6[0] {
                    false => MirroringMode::Horizontal,
                    true => MirroringMode::Vertical,
                },

                battery: f6[1],
                trainer: f6[2],
                ignore_mirroring: f6[3]
            }
        }
    })
}