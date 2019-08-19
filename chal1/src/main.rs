fn main() {
    println!("{:?}", hex_to_b64(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"))
}

fn hex_to_b64(hex: &[u8]) -> Vec<u8> {
    hex.chunks(2)
        .map(|c| ascii_to_binary(c[0]) * 16 + ascii_to_binary(c[1]))
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(bytes_to_b64)
        .collect::<Vec<u8>>()
}

fn ascii_to_binary(hex: u8) -> u8 {
    if hex >= 48 && hex <= 57 {
        hex - 48
    } else if hex >= 65 && hex <= 70 {
        hex - 55
    } else if hex >= 97 && hex <= 102 {
        hex - 87
    } else {
        unreachable!()
    }
}

fn bytes_to_b64(byte: &[u8]) -> Vec<u8> {
    let encodings = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_vec();
    let chunk: u32 = dbg!(((byte[0] as u32) << 16) + ((byte[1] as u32) << 8) + byte[2] as u32);

    let chars: [u8; 4] = [
        ((chunk & 0x00fc0000) >> 18) as u8,
        ((chunk & 0x0003f000) >> 12) as u8,
        ((chunk & 0x00000fc0) >> 6) as u8,
        ((chunk & 0x0000003f) >> 0) as u8,
    ];

    chars.iter().map(|c| encodings[*c as usize]).collect()
    // I am well aware the doesn't pad with "="
}

#[test]
fn test_hex_to_b64() {
    assert_eq!(hex_to_b64(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"), b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_vec())
}
