use base64;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use transpose;

fn main() {
    let input = File::open("6.txt").unwrap();
    decrypt(
        &base64::decode(
            &BufReader::new(input)
                .lines()
                .map(|x| x.unwrap())
                .collect::<Vec<_>>()
                .join(""),
        )
        .unwrap(),
    )
}

fn decrypt(file: &[u8]) {
    let mut keysizes_to_try: Vec<_> = (2usize..40usize)
        .map(|keysize| {
            (
                file.clone()
                    .chunks_exact(keysize)
                    .collect::<Vec<_>>()
                    .chunks_exact(2)
                    .map(|x| hamming(x[0], x[1]) / keysize as u32)
                    .sum::<u32>(),
                keysize,
            )
        })
        .collect();
    keysizes_to_try.sort();
    keysizes_to_try
        .iter()
        .take(4)
        .map(|(_distance, keysize)| *keysize)
        // (1..40) // temp
        .into_iter()
        .for_each(|keysize| {
            // i.e. for keysize 4:
            // file: [0, 1, 2, 3,
            //        4, 5, 6, 7,
            //        ...]
            // and we want
            // file: [0, 4, 8, 16, ...
            //        1, 5, 9, 17, ...
            //        ...]
            let height = file.len() / keysize; // Round down a bit
            let mut transposed_chunks = vec![0; keysize * height];
            transpose::transpose(
                &file[0..keysize * height],
                &mut transposed_chunks,
                keysize,
                height,
            );
            // println!("{:?}", &file[0..20]);
            // println!("{:?}", &transposed_chunks[0..20]);
            let chunks = transposed_chunks.chunks(height);
            let solved_chunks = find_the_xor(chunks);
            let key: Vec<u8> = solved_chunks.iter().map(|(_, _, k)| *k).collect();
            println!("Solving for keysize {} with key: {:?}", keysize, key);
            println!(
                "Got result {:?}",
                String::from_utf8(repeating_xor(file, &key))
            );
        });
}

fn repeating_xor(one: &[u8], two: &[u8]) -> Vec<u8> {
    one.iter()
        .zip(two.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn hamming(one: &[u8], two: &[u8]) -> u32 {
    assert_eq!(one.len(), two.len());
    one.iter()
        .zip(two)
        .map(|(a, b)| (*a ^ *b).count_ones() as u32)
        .sum()
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
        'j' | 'x' | 'q' | 'z' | '\n' => 0,
        _ => -2,
    };
}

fn score_ascii(string: Vec<u8>) -> i64 {
    string.into_iter().map(|x| score_ascii_byte(x)).sum()
}

fn find_the_xor<'a>(strings: impl Iterator<Item = &'a [u8]>) -> Vec<(i64, String, u8)> {
    strings
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
        .collect()
}

#[test]
fn test_hamming() {
    assert_eq!(hamming(b"this is a test", b"wokka wokka!!!"), 37)
}
