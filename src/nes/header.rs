use bitfield_struct::bitfield;
use std::fmt;

const NES_MAGIC: &[u8; 4] = b"NES\x1A";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderFormat {
    INes,  // iNES v1
    Nes2,  // NES 2.0
}

#[derive(Debug, Clone, Copy)]
pub enum HeaderParseError {
    TooShort,
    InvalidMagic,
}

impl fmt::Display for HeaderParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderParseError::TooShort => write!(f, "header too short"),
            HeaderParseError::InvalidMagic => write!(f, "invalid NES header magic"),
        }
    }
}

impl std::error::Error for HeaderParseError {}

#[derive(Debug, Clone, Copy)]
pub enum RamSize {
    Ines(u32),
    Nes2{ram: u32, nvram: u32},
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TimingMode {
    NTSC = 0,
    PAL = 1,
    MultipleRegions = 2,
    Dendy = 3,
}

impl TimingMode {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value & 0b11 {
            0 => Self::NTSC,
            1 => Self::PAL,
            2 => Self::MultipleRegions,
            3 => Self::Dendy,
            _ => unreachable!(),
        }
    }
}

// Backward compatible with iNes which uses D0 as Vs. System and D1 as PlayChoice-10
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConsoleType {
    NES = 0,
    VsSystem = 1,
    PlayChoice10 = 2,
    ExtendedConsole = 3,
}

impl ConsoleType {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value & 0b11 {
            0 => Self::NES,
            1 => Self::VsSystem,
            2 => Self::PlayChoice10,
            3 => Self::ExtendedConsole,
            _ => unreachable!(),
        }
    }
}

#[bitfield(u8)]
pub struct VsSystemType {
    #[bits(4)]
    pub ppu_type: u8,    // bits 0-3
    #[bits(4)]
    pub hardware_type: u8, // bits 4-7
}

#[bitfield(u8)]
pub struct ExtendedConsoleType {
    #[bits(4)]
    pub extended_console_type: u8,    // bits 0-3
    #[bits(4)]
    __: u8, // bits 4-7
}

#[bitfield(u8)]
pub struct Flags6 {
    pub nametable: bool,      // bit 0
    pub battery_backed: bool,  // bit 1
    pub trainer: bool,        // bit 2
    pub alternative_nametable: bool,    // bit 3
    #[bits(4)]
    pub mapper_low: u8,       // bits 4-7
}

#[bitfield(u8)]
pub struct Flags7 {
    #[bits(2)]
    pub console: ConsoleType,   // bit 0-1
    #[bits(2)]
    pub format: u8,           // bits 2-3 (should be 2 for NES 2.0)
    #[bits(4)]
    pub mapper_high: u8,      // bits 4-7
}

#[bitfield(u8)]
pub struct Flags9Nes2 {
    #[bits(4)]
    pub prg_rom_msb: u8,      // bits 0-3
    #[bits(4)]
    pub chr_rom_msb: u8,      // bits 4-7
}

#[bitfield(u8)]
pub struct Flags8Nes2 {
    #[bits(4)]
    pub submapper: u8,        // bits 0-3
    #[bits(4)]
    pub mapper_high2: u8,     // bits 4-7 (mapper bits 12-15)
}

#[bitfield(u8)]
pub struct Flags10Nes2 {
    #[bits(4)]
    pub prg_ram_shift: u8,    // bits 0-3
    #[bits(4)]
    pub prg_nvram_shift: u8,  // bits 4-7
}

#[bitfield(u8)]
pub struct Flags11Nes2 {
    #[bits(4)]
    pub chr_ram_shift: u8,    // bits 0-3
    #[bits(4)]
    pub chr_nvram_shift: u8,  // bits 4-7
}

#[bitfield(u8)]
pub struct Flags12Nes2 {
    #[bits(2)]
    pub timing_mode: TimingMode,    // bits 0-1
    #[bits(6)]
    __: u8,       // bits 2-7
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Flags13Nes2 {
    VsSystem(VsSystemType),
    ExtendedConsole(ExtendedConsoleType),
    UnusedForConsoleType(u8),
}
impl Flags13Nes2 {
    // This has to be a const fn
    const fn into_bits(self) -> u8 {
        match self {
            Flags13Nes2::VsSystem(vs) => vs.into_bits(),
            Flags13Nes2::ExtendedConsole(ec) => ec.into_bits(),
            Flags13Nes2::UnusedForConsoleType(value) => value,
        }
    }

    fn from_bits(value: u8, flags7: Flags7) -> Self {
        // When Byte 7 AND 3 =1: Vs. System Type
        if flags7.console() == ConsoleType::VsSystem {
            Flags13Nes2::VsSystem(VsSystemType::from_bits(value))
        // When Byte 7 AND 3 =3: Extended Console Type
        } else if flags7.console() == ConsoleType::ExtendedConsole {
            Flags13Nes2::ExtendedConsole(ExtendedConsoleType::from_bits(value))
        } else {
            Flags13Nes2::UnusedForConsoleType(value)
        }
    }
}

#[derive(Debug, Clone)]
pub struct InesHeader {
    /// Which header format this is (iNES v1 or NES 2.0)
    pub format: HeaderFormat,

    /// PRG ROM size in bytes
    pub prg_rom_size: u32,

    /// CHR ROM size in bytes
    pub chr_rom_size: u32,

    /// Mapper number (16-bit value for NES 2.0, 8-bit for iNES)
    pub mapper: u16,

    /// Submapper number (NES 2.0 only)
    pub submapper: u8,

    /// Whether the image contains a 512-byte trainer
    pub has_trainer: bool,

    /// PRG RAM size in bytes (interpreted according to format)
    pub prg_ram_size: RamSize,

    /// CHR RAM size in bytes (interpreted according to format)
    pub chr_ram_size: RamSize,

    /// Parsed flags from byte 6
    pub flags_6: Flags6,
    /// Parsed flags from byte 7
    pub flags_7: Flags7,
    /// Parsed flags from byte 9 (NES 2.0 specific)
    pub flags_9: Flags9Nes2,
    /// Parsed flags from byte 10 (NES 2.0 specific)
    pub flags_10: Flags10Nes2,
    /// Parsed flags from byte 11 (NES 2.0 specific)
    pub flags_11: Flags11Nes2,
    /// Parsed flags from byte 12 (NES 2.0 specific)
    pub flags_12: Flags12Nes2,
    /// Parsed flags from byte 13 (NES 2.0 specific)
    pub flags_13: Flags13Nes2,
}

impl InesHeader {
    /// Parse a 16-byte iNES/NES2 header from `bytes` (must be at least 16 bytes long).
    ///
    /// This will detect NES 2.0 via the magic bits in header[7] and populate the
    /// extended PRG/CHR sizes when present.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HeaderParseError> {
        if bytes.len() < 16 {
            return Err(HeaderParseError::TooShort);
        }

        if &bytes[0..4] != NES_MAGIC {
            return Err(HeaderParseError::InvalidMagic);
        }

        // Parse flags 6 (mapper low nibble and mirroring)
        let flags_6 = Flags6::from_bits(bytes[6]);
        // Parse flags 7 (mapper high nibble and format)
        let flags_7 = Flags7::from_bits(bytes[7]);
        // Parse flags 8 (NES 2.0 mapper/submapper)
        let flags_8 = Flags8Nes2::from_bits(bytes[8]);
        // Parse flags 9 (NES 2.0 extended ROM size MSBs)
        let flags_9 = Flags9Nes2::from_bits(bytes[9]);
        // Parse flags 10 (NES 2.0 RAM size shift counts)
        let flags_10 = Flags10Nes2::from_bits(bytes[10]);
        // Parse flags 11 (NES 2.0 RAM size shift counts)
        let flags_11 = Flags11Nes2::from_bits(bytes[11]);
        // Parse flags 12 (NES 2.0 timing mode)
        let flags_12 = Flags12Nes2::from_bits(bytes[12]);
        // Parse flags 13 (NES 2.0 console type)
        let flags_13 = Flags13Nes2::from_bits(bytes[13], flags_7);

        // Detect NES 2.0: bits 2-3 of byte 7 equal 2 (binary 10)
        let is_nes2 = flags_7.format() == 2;

        if is_nes2 {
            // Calculate extended ROM sizes using MSB from flags 9
            let prg_lsb = bytes[4] as u32;
            let chr_lsb = bytes[5] as u32;
            let prg_chunks = ((flags_9.prg_rom_msb() as u32) << 8) | prg_lsb;
            let chr_chunks = ((flags_9.chr_rom_msb() as u32) << 8) | chr_lsb;

            // Convert chunks to bytes:
            // PRG ROM: 16 KiB units
            // CHR ROM: 8 KiB units
            let prg_rom_size = prg_chunks * 16 * 1024;
            let chr_rom_size = chr_chunks * 8 * 1024;

            // NES 2.0 mapper combines bits from flags 6, 7, and 8
            let mapper = ((flags_8.mapper_high2() as u16) << 12) |
                     ((flags_7.mapper_high() as u16) << 8) |
                     ((flags_6.mapper_low() as u16) << 4);

            // Calculate PRG RAM size using shift count from flags 10
            // Size = 64 << shift count (in bytes), or 0 if shift count is 0
            let prg_ram_size = if flags_10.prg_ram_shift() == 0 {
                0
            } else {
                64 << flags_10.prg_ram_shift()  // NES 2.0 uses 64-byte units
            };

            let prg_nvram_size = if flags_10.prg_nvram_shift() == 0 {
                0
            } else {
                64 << flags_10.prg_nvram_shift()  // NES 2.0 uses 64-byte units
            };

            // Calculate CHR RAM size using shift count from flags 11
            let chr_ram_size = if flags_11.chr_ram_shift() == 0 {
                0
            } else {
                64 << flags_11.chr_ram_shift()  // NES 2.0 uses 64-byte units
            };

            let chr_nvram_size = if flags_11.chr_nvram_shift() == 0 {
                0
            } else {
                64 << flags_11.chr_nvram_shift()  // NES 2.0 uses 64-byte units
            };

            Ok(Self {
                format: HeaderFormat::Nes2,
                prg_rom_size,
                chr_rom_size,
                mapper,
                submapper: flags_8.submapper(),
                has_trainer: flags_6.trainer(),
                prg_ram_size: RamSize::Nes2 { ram: prg_ram_size, nvram: prg_nvram_size },
                chr_ram_size: RamSize::Nes2 { ram: chr_ram_size, nvram: chr_nvram_size },
                flags_6,
                flags_7,
                flags_9,
                flags_10,
                flags_11,
                flags_12,
                flags_13,
            })
        } else {
            // iNES format: simpler mapper and RAM size handling

            //Older versions of the iNES emulator ignored bytes 7-15, and several ROM management tools wrote messages in there.
            // Commonly, these will be filled with "DiskDude!", which results in 64 being added to the mapper number.
            // A general rule of thumb: if the last 4 bytes are not all zero, and the header is not marked for NES 2.0 format,
            // an emulator should either mask off the upper 4 bits of the mapper number or simply refuse to load the ROM
            let diskdude_signature = &bytes[12..16];
            let mapper = if diskdude_signature != [0, 0, 0, 0] {
                flags_6.mapper_low() as u16
            } else {
                ((flags_7.mapper_high() as u16) << 4) | (flags_6.mapper_low() as u16)
            };

            // Convert chunks to bytes:
            // PRG ROM: 16 KiB units
            // CHR ROM: 8 KiB units
            let prg_rom_size = (bytes[4] as u32) * 16 * 1024;
            let chr_rom_size = (bytes[5] as u32) * 8 * 1024;

            // In iNES format, byte 8 specifies RAM size in 8KB units
            let prg_ram_size = if bytes[8] == 0 {
                8 * 1024  // Default to 8KB if unspecified
            } else {
                (bytes[8] as u32) * 8 * 1024  // Convert to bytes
            };

            // In iNES format, if chr_rom_size is 0, we assume 8KB of CHR RAM as default 
            // the mapper will determine the CHR RAM size
            let chr_ram_size = if bytes[5] == 0 {
                8 * 1024  // Default to 8KB if unspecified
            } else {
                0  // CHR RAM not used
            };

            Ok(Self {
                format: HeaderFormat::INes,
                prg_rom_size,
                chr_rom_size,
                mapper,
                submapper: 0, // No submapper in iNES format
                has_trainer: flags_6.trainer(),
                prg_ram_size: RamSize::Ines(prg_ram_size),
                chr_ram_size: RamSize::Ines(chr_ram_size),
                flags_6,
                flags_7,
                flags_9,
                flags_10,
                flags_11,
                flags_12,
                flags_13,
            })
        }
    }
}