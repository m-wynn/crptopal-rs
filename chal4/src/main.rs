use hex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = File::open("file.txt").unwrap();
    find_the_xor(BufReader::new(input))
}

fn score_ascii_byte(c: u8) -> i64 {
    let c = if b'A' <= c && c <= b'Z' {
        c - b'A' + b'a'
    } else {
        c
    };
    return match c as char {
        'e' => 12,
        't' | 'a' | 'o' => 8,
        'i' | 'n' => 7,
        's' | 'h' | 'r' => 6,
        'd' | 'l' => 4,
        'c' | 'u' => 3,
        'm' | 'w' | 'f' | 'g' | 'y' | 'p' => 2,
        'b' | 'v' | 'k' | ' ' => 1,
        'j' | 'x' | 'q' | 'z' => 0,
        _ => -1,
    };
}

fn score_ascii(string: Vec<u8>) -> i64 {
    string.into_iter().map(|x| score_ascii_byte(x)).sum()
}

fn find_the_xor(strings: impl BufRead) {
    let answer = strings
        .lines()
        .map(|line| hex::decode(line.unwrap()).unwrap())
        .map(|string| {
            (0u8..std::u8::MAX)
                .map(|i| {
                    let test: Vec<u8> = string.iter().map(|a| a ^ i).collect();
                    if let Ok(str) = String::from_utf8(test.clone()) {
                        if str.is_ascii() {
                            return (score_ascii(test), str, i.clone());
                        }
                    }
                    (0, "Not Found".into(), 0u8)
                })
                .max()
                .unwrap()
        })
        .max();
    println!("{:?}", answer);
}
