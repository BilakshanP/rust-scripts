use std::{
    io,
    io::Write,
    fs
};

use super::base64;

pub fn append_to_file(path: &str, content: &[u8]) -> io::Result<()> {
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?
        .write_all(content)?;

    Ok(())
}

/// Reads non b64 data as b64 and gives vec
pub fn read_as_b64(path: &str) -> io::Result<Vec<u8>> {
    Ok(base64::encode(&fs::read(path)?))
}

/// Reads b64 data as non b64 and gives vec
pub fn read_from_b64(path: &str) -> io::Result<Vec<u8>> {
    Ok(base64::decode(&fs::read(path)?))
}

/// Reads non b64 data as b64 and gives str
pub fn read_as_b64_to_str(path: &str) -> io::Result<String> {
    Ok(base64::vec2str_encode(&fs::read(path)?))
}

/// Reads b64 data as non b64 and gives str
pub fn read_from_b64_to_str(path: &str) -> io::Result<String> {
    Ok(base64::vec2str_decode(&fs::read(path)?))
}

/// Takes non b64 data writes it as b64
pub fn write_as_b64(path: &str, content: &[u8]) -> io::Result<()> {
    fs::write(path, base64::encode(content))
}

/// Takes b64 data writes it as non b64
pub fn write_from_b64(path: &str, content: &[u8]) -> io::Result<()> {
    fs::write(path, base64::decode(content))
}