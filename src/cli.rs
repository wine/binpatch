use clap::Parser;

use crate::error::Error;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub file: String,

    #[arg(short, long)]
    signature: String,

    #[arg(short, long)]
    patch: String,
}

impl Cli {
    pub fn signature(&self) -> Result<Vec<u8>, Error> {
        to_vec(&self.signature)
    }

    pub fn patch(&self) -> Result<Vec<u8>, Error> {
        to_vec(&self.patch)
    }
}

fn to_vec(input: &str) -> Result<Vec<u8>, Error> {
    input.split(" ")
        .map(|s| u8::from_str_radix(&s, 16).map_err(|_| Error::InvalidBytes))
        .collect::<Result<Vec<_>, _>>()
}
