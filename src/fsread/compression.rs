use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    None,
    Fast,
    Balanced,
    Best,
    Maximum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    None,
    Lz4,
    Zstd,
    Brotli,
}

pub(crate) fn decompress(data: &[u8], codec: Codec) -> io::Result<Vec<u8>> {
    match codec {
        Codec::None => Ok(data.to_vec()),
        Codec::Lz4 => lz4_flex::decompress_size_prepended(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        Codec::Zstd => zstd::stream::decode_all(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        Codec::Brotli => {
            let mut decoder = brotli::Decompressor::new(data, 4096);
            let mut output = Vec::new();
            decoder.read_to_end(&mut output)?;
            Ok(output)
        }
    }
}
