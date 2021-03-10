use colored::*;
use pad::PadStr;
use std::fmt::{Display, Formatter, Result};

pub type U4 = u8;

pub trait U4Ops: Clone {
    fn to_u4(&self) -> Vec<U4>;
}

impl U4Ops for Vec<u8> {
    fn to_u4(&self) -> Vec<U4> {
        let mut v = Vec::new();
        for i in self {
            v.push(i / 16);
            v.push(i % 16);
        }
        return v;
    }
}

pub fn combine_u8(hsb: U4, lsb: U4) -> u8 {
    hsb * 16 + lsb
}

pub fn combine_u16(lsb: u8, hsb: u8) -> u16 {
    hsb as u16 * 256 + lsb as u16
}

static LOG_PADDING: usize = 7;

#[derive(std::cmp::PartialEq)]
pub enum ProgramStatus {
    Running,
    Exited,
}
pub struct Program {
    pub carry: bool,
    pub instructions: Vec<[U4; 8]>,
    pub memory: [u16; 0xFFFF],
    pub negative: bool,
    pub pointer: usize,
    pub overflow: bool,
    pub registers: [u16; 16],
    pub status: ProgramStatus,
    pub verbose: bool,
    pub zero: bool,
}

impl Program {
    pub fn from_u8(item: Vec<u8>) -> Program {
        let u4item = item.to_u4();
        let mut instructions: Vec<[U4; 8]> = Vec::new();
        let mut a: [U4; 8] = [0; 8];
        for i in 0..(item.len() / 4) {
            a.copy_from_slice(&u4item[(i * 8)..((i + 1) * 8)]);
            instructions.push(a);
        }
        Program {
            carry: false,
            instructions,
            memory: [0; 0xFFFF],
            negative: false,
            pointer: 0,
            overflow: false,
            registers: [0; 16],
            status: ProgramStatus::Running,
            verbose: false,
            zero: false,
        }
    }

    pub fn log_flow(&self, text: &str) {
        if self.verbose {
            println!(
                "{} {}",
                format!("{}", " Flow ".black().on_bright_purple()).pad(
                    LOG_PADDING + 11,
                    ' ',
                    pad::Alignment::Left,
                    false
                ),
                text.purple()
            );
        }
    }

    pub fn log_info(&self, text: &str) {
        if self.verbose {
            println!(
                "{} {}",
                format!("{}", " Info ".black().on_bright_yellow()).pad(
                    LOG_PADDING + 11,
                    ' ',
                    pad::Alignment::Left,
                    false
                ),
                text.yellow()
            );
        }
    }

    pub fn log_operation(&self, text: &str) {
        if self.verbose {
            println!(
                "{} {}",
                format!("{}", " Exec ".black().on_bright_blue()).pad(
                    LOG_PADDING + 11,
                    ' ',
                    pad::Alignment::Left,
                    false
                ),
                text.blue()
            );
        }
    }

    pub fn pass_error(&mut self, text: &str) {
        println!(
            "{} {}",
            format!("{}", " Error ".black().on_bright_red()).pad(
                LOG_PADDING + 11,
                ' ',
                pad::Alignment::Left,
                false
            ),
            text.red()
        );
        self.set_status(ProgramStatus::Exited);
    }

    pub fn set_status(&mut self, status: ProgramStatus) {
        match status {
            ProgramStatus::Running => self.log_flow("Program is running."),
            ProgramStatus::Exited => self.log_flow("The program will now exit."),
        }

        self.status = status;
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let len = self.instructions.len() / 16 + 1;
        let mut s = String::from("[Instructions]");
        for (i, arr) in self.instructions.iter().enumerate() {
            s.push_str(
                format!(
                    "\n{}: ",
                    format!("{:X}", i).pad(len, '0', pad::Alignment::Right, false)
                )
                .as_str(),
            );
            for j in arr {
                s.push(char::from_digit(*j as u32, 16).unwrap());
            }
        }
        write!(f, "{}", s)
    }
}
