use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader, Read};

use thiserror::Error;

use crate::nes::header::InesHeader;


#[derive(Error, Debug)]
pub enum RomParseError {
    #[error("header too short")]
    HeaderTooShort,
    #[error("invalid NES header magic")]
    HeaderInvalidMagic,
    #[error("invalid ROM size, inconsistent with header")]
    InvalidRomSize,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct Cartridge {
    pub ines_header: InesHeader,
    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub playchoice_inst_rom: Option<Vec<u8>>,
    pub player_prom: Option<Vec<u8>>,
}

impl Cartridge {
    pub fn load_rom_file(path: impl AsRef<Path>) -> Result<Self, RomParseError> {

        
        todo!()
    }

    pub fn load_rom_data<R: Read>(data: &mut R) -> Result<Self, RomParseError> {

        
        todo!()
    }
}