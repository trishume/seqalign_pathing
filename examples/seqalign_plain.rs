extern crate seqalign_pathing;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use seqalign_pathing::alignment_score;

fn file_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let af = file_bytes(&args[1]).unwrap();
    let bf = file_bytes(&args[2]).unwrap();
    let score = alignment_score(&af, &bf);
    println!("{:?}", score);
}
