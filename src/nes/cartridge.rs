use std::path::Path;
use std::io::Read;

use crate::nes::error::RomParseError;
use crate::nes::header::InesHeader;


const TRAINER_SIZE: usize = 512;
const HEADER_SIZE: usize = 16;

pub struct Cartridge {
    pub ines_header: InesHeader,
    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub misc_rom: Option<Vec<u8>>,
}

impl Cartridge {
    pub fn load_rom_file(path: impl AsRef<Path>) -> Result<Self, RomParseError> {
        let bytes = std::fs::read(path)?;
        let header = InesHeader::from_bytes(&bytes)?;

        let trainer_data = Self::extract_trainer(&bytes, &header)?;
        let (prg_rom, chr_rom) = Self::extract_prg_chr(&bytes, &header)?;
        let misc_rom = Self::extract_misc(&bytes, &header);

        Ok(Cartridge {
            ines_header: header,
            trainer: trainer_data,
            prg_rom,
            chr_rom,
            misc_rom,
        })
    }

    pub fn load_rom_data<R: Read>(data: &mut R) -> Result<Self, RomParseError> {
        let mut bytes = Vec::new();
        data.read_to_end(&mut bytes)?;

        let header = InesHeader::from_bytes(&bytes)?;

        let trainer_data = Self::extract_trainer(&bytes, &header)?;
        let (prg_rom, chr_rom) = Self::extract_prg_chr(&bytes, &header)?;
        let misc_rom = Self::extract_misc(&bytes, &header);

        Ok(Cartridge {
            ines_header: header,
            trainer: trainer_data,
            prg_rom,
            chr_rom,
            misc_rom,
        })
    }

    fn extract_trainer(data: &[u8], header: &InesHeader) -> Result<Option<Vec<u8>>, RomParseError> {
        if header.flags_6.trainer() {
            if data.len() < HEADER_SIZE + TRAINER_SIZE {
                return Err(RomParseError::InvalidRomSize);
            }
            Ok(Some(data[HEADER_SIZE..HEADER_SIZE + TRAINER_SIZE].to_vec()))
        } else {
            Ok(None)
        }
    }

    fn extract_prg_chr(data: &[u8], header: &InesHeader) -> Result<(Vec<u8>, Vec<u8>), RomParseError> {
        
        let trainer_offset = if header.flags_6.trainer() { TRAINER_SIZE } else { 0 };
        let prg_start = HEADER_SIZE + trainer_offset;
        let prg_end = prg_start + (header.prg_rom_size as usize);
        let chr_start = prg_end;
        let chr_end = chr_start + (header.chr_rom_size as usize);

        if data.len() < chr_end {
            return Err(RomParseError::InvalidRomSize);
        }

        let prg_rom = data[prg_start..prg_end].to_vec();
        let chr_rom = data[chr_start..chr_end].to_vec();

        Ok((prg_rom, chr_rom))
    }   

    fn extract_misc(data: &[u8], header: &InesHeader) -> Option<Vec<u8>> {
        // This data follows PRG and CHR ROMs, if present and is not indicated in the header.
        // This data depends on the console type and mapper type, so we will just extract it as raw bytes for now.
        let trainer_offset = if header.flags_6.trainer() { TRAINER_SIZE } else { 0 };
        let prg_start = HEADER_SIZE + trainer_offset;
        let prg_end = prg_start + (header.prg_rom_size as usize);
        let chr_start = prg_end;
        let chr_end = chr_start + (header.chr_rom_size as usize);

        if data.len() > chr_end {
            Some(data[chr_end..].to_vec())
        } else {
            None
        }
    }
}