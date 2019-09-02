use base64;
use crypto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = File::open("10.txt").unwrap();
    println!(
        "{}",
        String::from_utf8(decrypt(
            &base64::decode(
                &BufReader::new(input)
                    .lines()
                    .map(|x| x.unwrap())
                    .collect::<Vec<_>>()
                    .join(""),
            )
            .unwrap(),
            b"YELLOW SUBMARINE",
            &[0; 16],
        ))
        .unwrap()
    );
}

fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    let mut prev_block = iv;
    ciphertext.chunks(16).for_each(|ciphertext_block| {
        let mut decryptor = crypto::aes::ecb_decryptor(
            crypto::aes::KeySize::KeySize128,
            key,
            crypto::blockmodes::NoPadding,
        );
        let mut plaintext_block = [0; 16];
        decryptor
            .decrypt(
                &mut crypto::buffer::RefReadBuffer::new(&ciphertext_block),
                &mut crypto::buffer::RefWriteBuffer::new(&mut plaintext_block),
                true,
            )
            .unwrap();
        plaintext.append(
            &mut plaintext_block
                .iter()
                .zip(prev_block.iter())
                .map(|(a, b)| a ^ b)
                .collect::<Vec<u8>>(),
        );
        prev_block = &ciphertext_block;
    });
    return plaintext.to_vec();
}

fn encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    let mut prev_block = iv.to_vec();
    let mut ciphertext_block = [0; 16];
    plaintext.chunks(16).for_each(|plaintext_block| {
        let mut encryptor = crypto::aes::ecb_encryptor(
            crypto::aes::KeySize::KeySize128,
            key,
            crypto::blockmodes::NoPadding,
        );
        encryptor
            .encrypt(
                &mut crypto::buffer::RefReadBuffer::new(
                    &mut plaintext_block
                        .iter()
                        .zip(prev_block.iter())
                        .map(|(a, b)| a ^ b)
                        .collect::<Vec<u8>>(),
                ),
                &mut crypto::buffer::RefWriteBuffer::new(&mut ciphertext_block),
                true,
            )
            .unwrap();
        prev_block = ciphertext_block.to_vec();
        ciphertext.extend_from_slice(&ciphertext_block);
    });
    return ciphertext.to_vec();
}

#[test]
fn test_encrypt_decrypt() {
    let key = b"YELLOW SUBMARINE";
    let iv = &[0; 16];
    assert_eq!(
        decrypt(
            &encrypt(b"HELLO THERE!! GENERAL KENOBI!!!!", key, iv),
            key,
            iv
        ),
        b"HELLO THERE!! GENERAL KENOBI!!!!"
    );
}
