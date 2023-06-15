use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{Read, Write};

pub fn compress_data(data: &[u8], level: Compression) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), level);
    encoder.write_all(data)?;
    encoder.finish()
}

pub fn decompress_data(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder: GzDecoder<&[u8]> = GzDecoder::new(data);
    let mut decompressed: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}