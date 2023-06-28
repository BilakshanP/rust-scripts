```rust
// SHAKE128 algorithm
pub fn shake128(input_bytes: &[u8], output_byte_len: usize) -> Vec<u8> {
    keccak(1344, 256, input_bytes, 0x1F, output_byte_len)
}

// SHAKE256 algorithm
pub fn shake256(input_bytes: &[u8], output_byte_len: usize) -> Vec<u8> {
    keccak(1088, 512, input_bytes, 0x1F, output_byte_len)
}

// SHA3-224 algorithm
pub fn sha3_224(input_bytes: &[u8]) -> Vec<u8> {
    keccak(1152, 448, input_bytes, 0x06, 224 / 8)
}

// SHA3-256 algorithm
pub fn sha3_256(input_bytes: &[u8]) -> Vec<u8> {
    keccak(1088, 512, input_bytes, 0x06, 256 / 8)
}

// SHA3-384 algorithm
pub fn sha3_384(input_bytes: &[u8]) -> Vec<u8> {
    keccak(832, 768, input_bytes, 0x06, 384 / 8)
}

// SHA3-512 algorithm
pub fn sha3_512(input_bytes: &[u8]) -> Vec<u8> {
    keccak(576, 1024, input_bytes, 0x06, 512 / 8)
}

fn main() {
    let message = b"Hello, world!";

    let shake128_digest = shake128(message, 64);
    println!("SHAKE128: {}", to_hex(&shake128_digest));

    let shake256_digest = shake256(message, 64);
    println!("SHAKE256: {}", to_hex(&shake256_digest));

    let sha3_224_digest = sha3_224(message);
    println!("SHA3-224: {}", to_hex(&sha3_224_digest));

    let sha3_256_digest = sha3_256(message);
    println!("SHA3-256: {}", to_hex(&sha3_256_digest));

    let sha3_384_digest = sha3_384(message);
    println!("SHA3-384: {}", to_hex(&sha3_384_digest));

    let sha3_512_digest = sha3_512(message);
    println!("SHA3-512: {}", to_hex(&sha3_512_digest));
}
```