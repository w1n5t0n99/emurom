use std::path::Path;
use std::io::Read;

use crate::gb::error::RomParseError;
use crate::gb::header::GbHeader;


pub struct Cartridge {
    pub gb_header: GbHeader,
    pub rom_data: Vec<u8>,
}

impl Cartridge {
    pub fn load_rom_file(path: impl AsRef<Path>) -> Result<Self, RomParseError> {
        let bytes = std::fs::read(path)?;
        let header = GbHeader::from_bytes(&bytes)?;

        let rom_data = bytes[0x150..].to_vec();
        // most of the information in the header does not matter on real hardware
        // (the ROM’s size is determined only by the capacity of the ROM chip in the cartridge, not the header byte)
        let bank_size = 16 * 1024; // 16KB banks
        if bytes.len() % bank_size != 0 {
            return Err(RomParseError::InvalidRomSize);
        }

        Ok(Cartridge {
            gb_header: header,
            rom_data,
        })
    }

    pub fn load_rom_data<R: Read>(data: &mut R) -> Result<Self, RomParseError> {
        let mut bytes = Vec::new();
        data.read_to_end(&mut bytes)?;

        let header = GbHeader::from_bytes(&bytes)?;

        let rom_data = bytes[0x150..].to_vec();
        let bank_size = 16 * 1024; // 16KB banks
        if rom_data.len() % bank_size != 0 {
            return Err(RomParseError::InvalidRomSize);
        }

        Ok(Cartridge {
            gb_header: header,
            rom_data,
        })
    }
}