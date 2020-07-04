mod common;
mod compressor;
mod decompressor;
mod io;
mod model;

use anyhow::Result;
use clap::Clap;
use compressor::compress;
use decompressor::decompress;
use std::fs::File;
use std::io::Write;

#[derive(Clap)]
struct Opts {
    command: String,
    input: String,
    output: String,
}

fn main() -> Result<()> {
    let args: Opts = Opts::parse();
    let input = File::open(args.input)?;
    let mut output = File::create(args.output)?;
    match args.command.as_str() {
        "c" => {
            let compressed = compress(input)?;
            output.write_all(&compressed[..])?;
        }
        "d" => {
            let decompressed = decompress(input)?;
            output.write_all(&decompressed[..])?;
        }
        _ => {
            println!("Need 'c' or 'd'");
        }
    }

    Ok(())
}
