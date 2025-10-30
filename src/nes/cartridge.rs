use crate::nes::header::InesHeader;


pub struct Cartridge {
    pub ines_header: InesHeader,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}