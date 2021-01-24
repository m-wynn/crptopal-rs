use crypto::buffer::{self, BufferResult, ReadBuffer, WriteBuffer};
use rand::Rng;

fn main() {
    let (guessed, actual) = detect_mode(&generate_random_key());

    if guessed {
        println!("We guess ECB");
    } else {
        println!("We guess CBC");
    }

    if guessed == actual {
        println!("We were correct");
    } else {
        println!("We were wrong");
    }
}

fn detect_mode(key: &[u8]) -> (bool, bool) {
    let plaintext = &[b'a'; 32];
    let (actual_mode, cyphertext) = encryption_oracle(plaintext, &key);
    (detect_ecb(&cyphertext), actual_mode)
}

fn detect_ecb(line: &[u8]) -> bool {
    let mut chunks: Vec<_> = line.chunks(16).collect();
    let initial_len = chunks.len();
    chunks.sort();
    chunks.dedup();
    return chunks.len() < initial_len;
}

fn generate_random_key() -> [u8; 16] {
    rand::thread_rng().gen()
}

fn encryption_oracle(input: &[u8], key: &[u8]) -> (bool, Vec<u8>) {
    let mut rng = rand::thread_rng();

    // add 5-10 random bytes before and after the plaintext
    let prefix_len: u8 = rng.gen_range(5..=10);
    let suffix_len: u8 = rng.gen_range(5..=10);
    let mut message = vec![];
    message.extend((0..prefix_len).map(|_| rng.gen::<u8>()));
    message.extend(input.iter());
    message.extend((0..suffix_len).map(|_| rng.gen::<u8>()));

    // We want to return this to check our results
    let ecb: bool = rng.gen();

    let encryptor = if ecb == true {
        crypto::aes::ecb_encryptor(
            crypto::aes::KeySize::KeySize128,
            key,
            crypto::blockmodes::PkcsPadding,
        )
    } else {
        let iv: [u8; 16] = rng.gen();
        crypto::aes::cbc_encryptor(
            crypto::aes::KeySize::KeySize128,
            key,
            &iv,
            crypto::blockmodes::PkcsPadding,
        )
    };

    (ecb, encrypt(input, encryptor))
}

fn encrypt(
    plaintext: &[u8],
    mut encryptor: Box<dyn crypto::symmetriccipher::Encryptor>,
) -> Vec<u8> {
    let mut final_result = Vec::<u8>::new();
    let mut input = buffer::RefReadBuffer::new(plaintext);
    let mut buffer = [0; 4096];
    let mut output = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = encryptor.encrypt(&mut input, &mut output, true).unwrap();
        final_result.extend(
            output
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        if let BufferResult::BufferUnderflow = result {
            break;
        }
    }
    return final_result;
}

#[test]
fn test_detect_mode() {
    let plaintext = &[b'a'; 32];
    for _ in 0..1000 {
        let (actual_mode, cyphertext) = encryption_oracle(plaintext, &generate_random_key());
        if detect_ecb(&cyphertext) != actual_mode {
            panic!("We were wrong")
        }
    }
}
