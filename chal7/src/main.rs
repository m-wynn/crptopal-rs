use base64;
use crypto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = File::open("7.txt").unwrap();
    decrypt(
        &base64::decode(
            &BufReader::new(input)
                .lines()
                .map(|x| x.unwrap())
                .collect::<Vec<_>>()
                .join(""),
        )
        .unwrap(),
        b"YELLOW SUBMARINE",
    );
}

fn decrypt(file: &[u8], key: &[u8]) {
    let mut buffer = [0; 4096];
    let mut output = crypto::buffer::RefWriteBuffer::new(&mut buffer);
    let mut input = crypto::buffer::RefReadBuffer::new(file);
    crypto::aes::ecb_decryptor(
        crypto::aes::KeySize::KeySize128,
        key,
        crypto::blockmodes::NoPadding,
    )
    .decrypt(&mut input, &mut output, true)
    .unwrap();
    println!("{}", String::from_utf8(buffer.to_vec()).unwrap());
}
