mod cli;
mod error;

use std::{fs, ops::Range};

use clap::Parser;
use goblin::{elf::Elf, elf64::program_header::PT_LOAD};

use cli::Cli;
use error::Error;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let signature = cli.signature()?;
    let patch = cli.patch()?;
    let patch = apply_patch_to_signature(&signature, &patch);

    let mut data = fs::read(&cli.file).map_err(Error::FileRead)?;
    let elf = Elf::parse(&data).map_err(Error::ParseElf)?;

    let patch_location = find_patch_location(&data, &elf, &signature)?;
    data[patch_location].copy_from_slice(&patch);

    fs::write(cli.file, data).map_err(Error::FileWrite)?;

    Ok(())
}

fn find_patch_location(data: &[u8], elf: &Elf, signature: &[u8]) -> Result<Range<usize>, Error> {
    elf.program_headers
        .iter()
        .filter(|ph| ph.p_type == PT_LOAD)
        .filter(|ph| ph.is_executable())
        .find_map(|ph| {
            let ph_range = ph.file_range();
            let section = &data[ph_range.start..ph_range.end];
            section
                .windows(signature.len())
                .position(|w| w == signature)
                .map(|addr| ph_range.start + addr)
                .map(|addr| addr..(addr + signature.len()))
        })
        .ok_or(Error::PatchLocationNotFound)
}

fn apply_patch_to_signature(signature: &[u8], patch: &[u8]) -> Vec<u8> {
    signature
	.iter()
        .enumerate()
        .map(|(i, &byte)| {
            if i < patch.len() && patch[i] != 0x0 {
                patch[i]
            } else {
                byte
            }
        })
        .collect()
}
