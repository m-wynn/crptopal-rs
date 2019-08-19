use hex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = File::open("8.txt").unwrap();
    &BufReader::new(input)
        .lines()
        .map(|x| detect_ecb(&hex::decode(x.unwrap()).unwrap()))
        .enumerate()
        .filter(|(_, x)| *x == true)
        .for_each(|(i, _)| println!("Line {} is the ecb encrypted one", i));
}

fn detect_ecb(line: &[u8]) -> bool {
    let mut chunks: Vec<_> = line.chunks(16).collect();
    let initial_len = chunks.len();
    chunks.sort();
    chunks.dedup();
    return chunks.len() < initial_len;
}
