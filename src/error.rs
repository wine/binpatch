use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid bytes")]
    InvalidBytes,
    
    #[error("error reading file")]
    FileRead(io::Error),

    #[error("error parsing elf")]
    ParseElf(goblin::error::Error),

    #[error("patch location not found")]
    PatchLocationNotFound,

    #[error("error writing file")]
    FileWrite(io::Error),
}
