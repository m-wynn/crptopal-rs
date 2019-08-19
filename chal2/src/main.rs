use hex;

fn main() {
    println!("Hello, world!");
}

fn fixed_xor(one: &[u8], two: &[u8]) -> Vec<u8> {
    one.iter()
        .zip(two)
        .map(|(a, b)| (a & !b) | (!a & b))
        .collect()
}

#[test]
fn test_fixed_xor() {
    assert_eq!(
        fixed_xor(
            &hex::decode("1c0111001f010100061a024b53535009181c").unwrap(),
            &hex::decode("686974207468652062756c6c277320657965").unwrap()
        ),
        hex::decode("746865206b696420646f6e277420706c6179").unwrap()
    );
}
