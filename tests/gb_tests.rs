use std::path::PathBuf;


fn get_file_path(file_name: &str) -> String {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = manifest_dir.join("tests").join("test_data").join(file_name);
    println!("{:?}", file_path);
    file_path.to_str().unwrap().to_string()
}

#[test]
fn test_gb_rom() {
    let rom_path = get_file_path("gb_cpu_instrs.gb");
    let cartridge = emurom::gb::cartridge::Cartridge::load_rom_file(&rom_path);

    assert!(cartridge.is_ok());

    let cartridge = &cartridge.unwrap();

    assert_eq!(cartridge.gb_header.cartridge_type, emurom::gb::header::CartridgeType::MBC1, "Cartridge type mismatch");
    assert_eq!(cartridge.gb_header.has_ram(), false, "ROM size mismatch"); // No RAM
}