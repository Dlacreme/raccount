use super::super::chain;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Result};

pub fn write(path: String, chain: &chain::chain::Chain) -> Result<()> {

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(path.as_str())
        .unwrap();

    for (_i, block) in chain.blocks.iter().enumerate() {
        writeln!(file, "{}", serde_json::to_string(&block).unwrap())?
    }

    Ok(())
}

pub fn read(path: String) -> Result<()> {
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        println!("{}", line?);
    }
    Ok(())
}