//! More information at http://wiki.nesdev.com/w/index.php/INES

use bitvec::prelude::*;
use std::{error::Error, fs};

#[derive(Debug)]
enum ConsoleType {
    NES,
    VsSystem,
    PlayChoice,
}

#[derive(Debug)]
enum MirroringMode {
    Vertical,
    Horizontal,
    FourScreen,
}

#[derive(Debug)]
struct CartridgeHeader {
    prg_rom_size: u8,
    prg_ram_size: u8,
    chr_rom_size: Option<u8>,
    mapper: u8,
    mirroring: MirroringMode,
    battery: bool,
    trainer: bool,
    console_type: ConsoleType,
    tv_system: bool,
}

#[derive(Debug)]
pub struct Cartridge {
    header: CartridgeHeader,
}

fn to_u8(bits: &[u8]) -> u8 {
    bits.iter().fold(0, |result, &bit| (result << 1) ^ bit)
}

fn check_header(file: &Vec<u8>) -> Result<(), &str> {
    let header = file.get(0..4);
    match header {
        Some([b'N', b'E', b'S', 0x1A]) => Ok(()),
        _ => Err("File does not contain valid NES header"),
    }
}

pub fn read_file(path: &str) -> Result<Cartridge, Box<dyn Error + 'static>> {
    let file = fs::read(path)?;
    check_header(&file)?;

    let f6 = BitArray::<Lsb0, _>::new(file[6]);
    let f7 = BitArray::<Lsb0, _>::new(file[7]);
    let f9 = BitArray::<Lsb0, _>::new(file[9]);

    if !f7[2] && f7[3] {
        panic!("Nabby doesn't support the NES2.0 format, yet")
    }

    let mapper_lower = f6
        .get(4..8)
        .unwrap()
        .iter()
        .map(|b| *b as u8)
        .rev()
        .collect::<Vec<u8>>();
    let mut mapper_upper = f7
        .get(4..8)
        .unwrap()
        .iter()
        .map(|b| *b as u8)
        .rev()
        .collect::<Vec<u8>>();

    mapper_upper.extend(mapper_lower);

    let mapper = to_u8(&mapper_upper);

    let mut cart = Cartridge {
        header: CartridgeHeader {
            prg_rom_size: file[4],
            prg_ram_size: match file[8] {
                0 => 1,
                _ => file[4],
            },
            chr_rom_size: match file.get(5) {
                Some(0) | None => None,
                _ => Some(file[5]),
            },
            mapper,
            mirroring: match f6[0] {
                false => MirroringMode::Horizontal,
                true => MirroringMode::Vertical,
            },

            battery: f6[1],
            trainer: f6[2],

            console_type: ConsoleType::NES,
            tv_system: f9[0],
        },
    };

    if f6[3] {
        cart.header.mirroring = MirroringMode::FourScreen;
    }

    if f7[0] {
        cart.header.console_type = ConsoleType::VsSystem;
    } else if f7[1] {
        cart.header.console_type = ConsoleType::PlayChoice;
    }

    Ok(cart)
}
