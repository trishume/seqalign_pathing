extern crate seqalign_pathing;
extern crate bio;

use bio::io::fasta::Reader;
use seqalign_pathing::alignment_score;
use std::cmp::min;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 4);
    let af = Reader::from_file(&args[1]).unwrap().records().next().unwrap().unwrap();
    let bf = Reader::from_file(&args[2]).unwrap().records().next().unwrap().unwrap();
    let ab = af.seq();
    let bb = bf.seq();
    let prefix: usize = args[3].parse().unwrap();
    let score = alignment_score(&ab[0..min(prefix,ab.len())], &bb[0..min(prefix,bb.len())]);
    println!("{:?}", score);
}
