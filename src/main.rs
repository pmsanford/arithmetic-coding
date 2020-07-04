mod common;
mod compressor;
mod decompressor;
mod io;
mod model;

use anyhow::Result;
use clap::Clap;
use compressor::compress;
use std::fs::File;
use std::io::Write;

#[derive(Clap)]
struct Opts {
    input: String,
    output: String,
}

fn main() -> Result<()> {
    let args: Opts = Opts::parse();
    let input = File::open(args.input)?;
    let compressed = compress(input)?;
    let mut output = File::create(args.output)?;
    output.write_all(&compressed[..])?;

    Ok(())
}
