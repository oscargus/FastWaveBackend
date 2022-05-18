use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::collections::BTreeMap;

use num::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct Timestamp{
    file_offset: u64,
    timestamp:   BigInt
}

struct Signal {
    name          : String,
    timeline      : BTreeMap<BigInt, BigInt>,
    children_arena: Vec<usize>,
    parent_index  : usize
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let space = " ".as_bytes()[0];

    let file       = File::open(&args.path)?;
    let mut reader = io::BufReader::new(file);

    let mut buffer = Vec::<u8>::new();
    let mut word_count = 0u64;

    while {
        let bytes_read = reader.read_until(space, &mut buffer).unwrap();
        bytes_read > 0
    } {
        word_count += 1;
    }
    dbg!(word_count);

    Ok(())
}
