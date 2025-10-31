use thiserror::Error;


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