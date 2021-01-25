use crypto::buffer::{self, BufferResult, ReadBuffer, WriteBuffer};
use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref KEY: [u8; 16] = rand::thread_rng().gen();
}

fn main() {
    let (block_size, total_size) = detect_block();
    let result = detect_bytes(block_size, total_size);
    println!("{}", String::from_utf8(result).unwrap());
}

fn detect_block() -> (usize, usize) {
    let base_size = encryption_oracle(&[]).len();
    for i in 1..512 {
        let test_vec = vec![b'A'; i];
        let test_size = encryption_oracle(&test_vec).len();
        if test_size > base_size {
            return (test_size - base_size, base_size);
        }
    }
    panic!("Unable to find block size");
}

fn detect_bytes(block_size: usize, total_size: usize) -> Vec<u8> {
    // A = filler character 'A'
    // X = character to iterate over to guess
    // K = known secret character we can placein our input
    // U = unknown secret character, beginning of secret blocks automatically appended by encryption_oracle
    // In the first run we determine `encrypt([A, A, A, A, ... A, U | <secret block>...])`
    // And we test possible X values `encrypt([A, A, A, A, ... A, X | <secret block> | <secret block>...])`
    // Until we find X = U
    // We shrink our padding to find `encrypt([A, A, A, ... A, U, U | <secret block> | <secret block>...])`
    // The first letter is K, so try `encrypt([A, A, A, ... A, K, X | <secret block> | <secret block>...])`
    // Until we find X to be the second U.
    // Repeat for the entire block.
    // We find the whole first block `encrypt([K, K, K, ..block.. K | <secret block> | <secret block> ...])`
    //
    // To find the second block, we repeat and use the first block for padding again
    // We fill the second block with what we know [A, A, A, A, ... K | K, K, ..block.. K, U | <secret block> | <secret block> ...]
    // We compare this, just like the first round [A, A, A, A, ... K | K, K, ..block.. K, X | <secret block> | <secret block> ...]
    let mut known = vec![];
    for i in 1..=total_size {
        let padding_prefix = vec![b'A'; block_size - (i % block_size)];
        let target = encryption_oracle(&padding_prefix);
        for test_char in 0..=u8::MAX {
            let test_vec = [&padding_prefix[..], &known[..], &[test_char]].concat();
            let oracle_output = encryption_oracle(&test_vec);

            // round up to next block
            let block_len = ((i + block_size) / block_size) * block_size;

            // If the block is the same between our guessed character and the "target" without the
            // guess, we are all set for that character.
            if oracle_output[0..block_len] == target[0..block_len] {
                if test_char != 0 {
                    known.push(test_char);
                }
                break;
            }
        }
    }
    known
}

fn encryption_oracle(input: &[u8]) -> Vec<u8> {
    const APPEND: &[u8] = b"Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
                    aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
                    dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
                    YnkK";
    let decoded = &base64::decode(APPEND).unwrap();

    let final_input = [input, decoded].concat();

    let encryptor = crypto::aes::ecb_encryptor(
        crypto::aes::KeySize::KeySize128,
        &*KEY,
        crypto::blockmodes::PkcsPadding,
    );
    encrypt(&final_input, encryptor)
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
