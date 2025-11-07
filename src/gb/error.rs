use thiserror::Error;


#[derive(Error, Debug)]
pub enum RomParseError {
    #[error("header too short")]
    HeaderTooShort,
    #[error("invalid Nintendo logo")]
    InvalidLogo,
    #[error("invalid cartridge type")]
    InvalidCartridgeType,
    #[error("invalid header checksum")]
    InvalidHeaderChecksum,
    #[error("invalid ROM size")]
    InvalidRomSize,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
