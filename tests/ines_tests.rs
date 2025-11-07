use std::path::PathBuf;


fn get_file_path(file_name: &str) -> String {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = manifest_dir.join("tests").join("test_data").join(file_name);
    println!("{:?}", file_path);
    file_path.to_str().unwrap().to_string()
}

#[test]
fn test_ines_rom_header() {
    let rom_path = get_file_path("nes_nestest.nes");
    let cartridge = emurom::nes::cartridge::Cartridge::load_rom_file(&rom_path);

    assert!(cartridge.is_ok());

    let header = &cartridge.unwrap().ines_header;
    assert_eq!(header.format, emurom::nes::header::HeaderFormat::INes, "Header format mismatch");
    assert_eq!(header.prg_rom_size, 16*1024, "PRG ROM size mismatch"); // 16KB PRG ROM
    assert_eq!(header.chr_rom_size, 8*1024, "CHR ROM size mismatch");  // 8KB CHR ROM
    assert_eq!(header.flags_6.nametable(), false, "Nametable mirroring mismatch"); // H mirroring
    assert_eq!(header.flags_6.battery_backed(), false, "Battery backed mismatch");
    assert_eq!(header.flags_6.trainer(), false, "Trainer mismatch");
    assert_eq!(header.mapper, 0, "Mapper mismatch"); // No mapper
    assert_eq!(header.submapper, 0, "Submapper mismatch"); // No submapper
}

#[test]
fn test_nes2_rom_header() {
    let rom_path = get_file_path("nes_mmc3bigchrram.nes");
    let cartridge = emurom::nes::cartridge::Cartridge::load_rom_file(&rom_path);

    assert!(cartridge.is_ok());

    // test chr ram size parsing
    let header = &cartridge.unwrap().ines_header;
    assert_eq!(header.format, emurom::nes::header::HeaderFormat::Nes2, "Header format mismatch");
    assert_eq!(header.prg_rom_size, 64*1024, "PRG ROM size mismatch"); // 32KB PRG ROM
    assert_eq!(header.chr_rom_size, 0, "CHR ROM size mismatch");  // 0KB CHR ROM
    assert_eq!(header.chr_ram_size, emurom::nes::header::RamSize::Nes2{ram: 32*1024, nvram: 0}, "CHR RAM size mismatch");  // 32KB CHR RAM
    assert_eq!(header.flags_6.nametable(), false, "Nametable mirroring mismatch"); // H mirroring
    assert_eq!(header.flags_6.battery_backed(), false, "Battery backed mismatch");
    assert_eq!(header.flags_6.trainer(), false, "Trainer mismatch");
    assert_eq!(header.mapper, 4, "Mapper mismatch"); // No mapper
    assert_eq!(header.submapper, 0, "Submapper mismatch"); // No submapper

    // test large mapper and sub mapper parsing
    let rom_path = get_file_path("nes_34_test_2.nes");
    let cartridge = emurom::nes::cartridge::Cartridge::load_rom_file(&rom_path);

    assert!(cartridge.is_ok());

    let header = &cartridge.unwrap().ines_header;
    assert_eq!(header.format, emurom::nes::header::HeaderFormat::Nes2, "Header format mismatch");
    assert_eq!(header.prg_rom_size, 256*1024, "PRG ROM size mismatch"); // 32KB PRG ROM
    assert_eq!(header.chr_rom_size, 0, "CHR ROM size mismatch");  // 0KB CHR ROM
    assert_eq!(header.prg_ram_size, emurom::nes::header::RamSize::Nes2{ram: 8*1024, nvram: 0}, "PRG RAM size mismatch");  // 8KB PRG/Work RAM
    assert_eq!(header.chr_ram_size, emurom::nes::header::RamSize::Nes2{ram: 8*1024, nvram: 0}, "CHR RAM size mismatch");  // 8KB CHR RAM
    assert_eq!(header.flags_6.nametable(), false, "Nametable mirroring mismatch"); // H mirroring
    assert_eq!(header.flags_6.battery_backed(), false, "Battery backed mismatch");
    assert_eq!(header.flags_6.trainer(), false, "Trainer mismatch");
    assert_eq!(header.mapper, 34, "Mapper mismatch"); // No mapper
    assert_eq!(header.submapper, 2, "Submapper mismatch"); // No submapper
}

#[test]
pub fn test_cartridge_load_data() {
    let rom_path = get_file_path("nes_nestest.nes");
    let mut file = std::fs::File::open(&rom_path).expect("Failed to open ROM file");
    let cartridge = emurom::nes::cartridge::Cartridge::load_rom_data(&mut file);

    assert!(cartridge.is_ok());
    let cartridge = cartridge.unwrap();

    assert_eq!(cartridge.prg_rom.len(), cartridge.ines_header.prg_rom_size as usize, "PRG ROM size mismatch");
    assert_eq!(cartridge.chr_rom.len(), cartridge.ines_header.chr_rom_size as usize, "CHR ROM size mismatch");
}