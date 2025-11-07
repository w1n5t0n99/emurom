use bitfield_struct::bitfield;

use crate::gb::error::RomParseError;


/// Entry point and Nintendo logo from 0x104-0x133
const GB_LOGO: &[u8; 48] = &[
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderFormat {
    GB,   // Original Game Boy
    GBC,  // Game Boy Color
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CartridgeType {
    RomOnly = 0x00,
    MBC1 = 0x01,
    MBC1Ram = 0x02,
    MBC1RamBattery = 0x03,
    MBC2 = 0x05,
    MBC2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    MMM01 = 0x0B,
    MMM01Ram = 0x0C,
    MMM01RamBattery = 0x0D,
    MBC3TimerBattery = 0x0F,
    MBC3TimerRamBattery = 0x10,
    MBC3 = 0x11,
    MBC3Ram = 0x12,
    MBC3RamBattery = 0x13,
    MBC5 = 0x19,
    MBC5Ram = 0x1A,
    MBC5RamBattery = 0x1B,
    MBC5Rumble = 0x1C,
    MBC5RumbleRam = 0x1D,
    MBC5RumbleRamBattery = 0x1E,
    MBC6 = 0x20,
    MBC7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTAMA5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF,
}

impl CartridgeType {
    pub fn into_bits(self) -> u8 {
        self as _
    }

    pub fn from_bits(value: u8) -> Option<Self> {
        use CartridgeType::*;
        match value {
            0x00 => Some(RomOnly),
            0x01 => Some(MBC1),
            0x02 => Some(MBC1Ram),
            0x03 => Some(MBC1RamBattery),
            0x05 => Some(MBC2),
            0x06 => Some(MBC2Battery),
            0x08 => Some(RomRam),
            0x09 => Some(RomRamBattery),
            0x0B => Some(MMM01),
            0x0C => Some(MMM01Ram),
            0x0D => Some(MMM01RamBattery),
            0x0F => Some(MBC3TimerBattery),
            0x10 => Some(MBC3TimerRamBattery),
            0x11 => Some(MBC3),
            0x12 => Some(MBC3Ram),
            0x13 => Some(MBC3RamBattery),
            0x19 => Some(MBC5),
            0x1A => Some(MBC5Ram),
            0x1B => Some(MBC5RamBattery),
            0x1C => Some(MBC5Rumble),
            0x1D => Some(MBC5RumbleRam),
            0x1E => Some(MBC5RumbleRamBattery),
            0x20 => Some(MBC6),
            0x22 => Some(MBC7SensorRumbleRamBattery),
            0xFC => Some(PocketCamera),
            0xFD => Some(BandaiTAMA5),
            0xFE => Some(HuC3),
            0xFF => Some(HuC1RamBattery),
            _ => None,
        }
    }
}

#[bitfield(u8)]
pub struct GbcFlags {
    #[bits(7)]
    pub _reserved: u8,
    pub gbc_support: bool,  // true if game supports GBC features
}

#[bitfield(u8)]
pub struct SgbFlags {
    #[bits(7)]
    pub _reserved: u8,
    pub sgb_support: bool,  // true if game supports SGB features
}

#[derive(Debug, Clone)]
pub struct GbHeader {
    /// Entry point (usually 0x00 0xC3 0x50 0x01)
    pub entry_point: [u8; 4],
    
    /// Nintendo logo bitmap (must match known pattern)
    pub nintendo_logo: [u8; 48],
    
    /// Game title (upper-case ASCII)
    pub title: String,
    
    /// Manufacturer code (if newer game) or title continuation (if older game)
    pub manufacturer_code: Option<String>,
    
    /// GBC support flags
    pub gbc_flags: GbcFlags,
    
    /// New licensee code (used if old_licensee_code is 0x33)
    pub new_licensee_code: [u8; 2],
    
    /// SGB support flags
    pub sgb_flags: SgbFlags,
    
    /// Cartridge hardware type
    pub cartridge_type: CartridgeType,
    
    /// ROM size in bytes
    pub rom_size: u32,
    
    /// RAM size in bytes
    pub ram_size: u32,
    
    /// Destination code (0x00 = Japan, 0x01 = Non-Japan)
    pub destination: u8,
    
    /// Old licensee code (if 0x33, use new_licensee_code instead)
    pub old_licensee_code: u8,
    
    /// Mask ROM version number
    pub version: u8,
    
    /// Header checksum
    pub header_checksum: u8,
    
    /// Global checksum
    pub global_checksum: u16,
}

impl GbHeader {
    /// Parse a Game Boy ROM header from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RomParseError> {
        if bytes.len() < 0x150 {
            return Err(RomParseError::HeaderTooShort);
        }

        // Verify Nintendo logo
        if &bytes[0x104..0x134] != GB_LOGO {
            return Err(RomParseError::InvalidLogo);
        }

        // Parse title and manufacturer code
        let mut title = String::new();
        let mut manufacturer_code = None;
        let title_end = if bytes[0x143] == 0x80 || bytes[0x143] == 0xC0 {
            // If GBC flag is present, title ends at 0x13F
            0x13F
        } else {
            // Otherwise title can extend to 0x143
            0x143
        };

        // Convert title bytes to string, stopping at first 0x00 or end
        for &byte in &bytes[0x134..=title_end] {
            if byte == 0 {
                break;
            }
            title.push(byte as char);
        }

        // If title is short enough, remaining bytes are manufacturer code
        if title_end == 0x13F {
            let mfg = String::from_utf8_lossy(&bytes[0x13F..0x143]).to_string();
            if !mfg.trim().is_empty() {
                manufacturer_code = Some(mfg);
            }
        }

        // Parse GBC and SGB flags
        let gbc_flags = GbcFlags::from_bits(bytes[0x143]);
        let sgb_flags = SgbFlags::from_bits(bytes[0x146]);

        // Parse cartridge type
        let cart_type = CartridgeType::from_bits(bytes[0x147])
            .ok_or(RomParseError::InvalidCartridgeType)?;

        // Calculate ROM size (32KB << shift)
        let rom_shift = bytes[0x148];
        let rom_size = 32 * 1024 * (1 << rom_shift);

        // Calculate RAM size
        let ram_size = match bytes[0x149] {
            0x00 => 0,          // No RAM
            0x01 => 2 * 1024,   // 2 KB
            0x02 => 8 * 1024,   // 8 KB
            0x03 => 32 * 1024,  // 32 KB
            0x04 => 128 * 1024, // 128 KB
            0x05 => 64 * 1024,  // 64 KB
            _ => 0,             // Unknown value, assume no RAM
        };

        // Verify header checksum
        let mut checksum: u8 = 0;
        for &byte in &bytes[0x134..0x14D] {
            checksum = checksum.wrapping_sub(byte).wrapping_sub(1);
        }
        if checksum != bytes[0x14D] {
            return Err(RomParseError::InvalidHeaderChecksum);
        }

        // Parse global checksum (verify only if requested)
        let global_checksum = u16::from_be_bytes([bytes[0x14E], bytes[0x14F]]);

        Ok(Self {
            entry_point: bytes[0x100..0x104].try_into().unwrap(),
            nintendo_logo: bytes[0x104..0x134].try_into().unwrap(),
            title,
            manufacturer_code,
            gbc_flags,
            new_licensee_code: bytes[0x144..0x146].try_into().unwrap(),
            sgb_flags,
            cartridge_type: cart_type,
            rom_size,
            ram_size,
            destination: bytes[0x14A],
            old_licensee_code: bytes[0x14B],
            version: bytes[0x14C],
            header_checksum: bytes[0x14D],
            global_checksum,
        })
    }

    /// Returns true if this is a Game Boy Color enhanced game
    pub fn is_gbc(&self) -> bool {
        self.gbc_flags.gbc_support()
    }

    /// Returns true if this game supports Super Game Boy features
    pub fn is_sgb(&self) -> bool {
        self.sgb_flags.sgb_support()
    }

    /// Returns true if this cartridge has battery-backed RAM
    pub fn has_battery(&self) -> bool {
        use CartridgeType::*;
        matches!(
            self.cartridge_type,
            MBC1RamBattery | MBC2Battery | RomRamBattery | MMM01RamBattery |
            MBC3TimerBattery | MBC3TimerRamBattery | MBC3RamBattery |
            MBC5RamBattery | MBC5RumbleRamBattery | MBC7SensorRumbleRamBattery |
            HuC1RamBattery
        )
    }

    /// Returns true if this cartridge has RAM (battery-backed or not)
    pub fn has_ram(&self) -> bool {
        self.ram_size > 0
    }

    /// Returns true if this is a Japanese game
    pub fn is_japanese(&self) -> bool {
        self.destination == 0x00
    }
}
