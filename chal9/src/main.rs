fn main() {
    println!("Hello, world!");
}

fn pkcs7(str: &str, len: usize) -> String {
    format!("{:\x04<1$}", str, len)
}

#[test]
fn test_pkcs7() {
    assert_eq!(
        pkcs7("YELLOW SUBMARINE", 20),
        "YELLOW SUBMARINE\x04\x04\x04\x04"
    );
}
