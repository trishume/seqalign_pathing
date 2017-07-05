extern crate seqalign_pathing;
extern crate bio;

use bio::io::fasta::Reader;
use seqalign_pathing::alignment_score;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let af = Reader::from_file(&args[1]).unwrap().records().next().unwrap().unwrap();
    let bf = Reader::from_file(&args[2]).unwrap().records().next().unwrap().unwrap();
    let score = alignment_score(af.seq(), bf.seq());
    println!("{:?}", score);
}
