use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};

use crate::nes::error::RomParseError;
use crate::nes::header::InesHeader;


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
        let bytes = std::fs::read(path)?;
        let header = InesHeader::from_bytes(&bytes)?;

        
        
        todo!()
    }

    pub fn load_rom_data<R: Read + Seek>(data: &mut R) -> Result<Self, RomParseError> {

        
        todo!()
    }
}