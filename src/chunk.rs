use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
    io::{BufRead, BufReader, Read},
};

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::{chunk_type::ChunkType, Error, Result};

pub struct Chunk {
    length: u32,

    chunk_type: ChunkType,
    data: Vec<u8>,

    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Self::crc_checksum(&chunk_type, &data);
        Self::build(data.len() as u32, chunk_type, data, crc)
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.data.clone()).map_err(Into::into)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    pub fn read_chunk(reader: &mut BufReader<&[u8]>) -> Result<Chunk> {
        let mut buffer = [0; 4];

        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer)?;
        let chunk_type = buffer.try_into()?;

        let mut data = vec![0; length as usize];
        reader.read_exact(&mut data)?;

        reader.read_exact(&mut buffer)?;
        let crc = u32::from_be_bytes(buffer);

        if crc != Self::crc_checksum(&chunk_type, &data) {
            return Err("invalid chunk".into());
        }

        Ok(Self::build(length, chunk_type, data, crc))
    }
}

impl Chunk {
    const CRC_32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

    fn crc_checksum(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let bytes: Vec<_> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        Self::CRC_32.checksum(&bytes)
    }

    fn build(length: u32, chunk_type: ChunkType, data: Vec<u8>, crc: u32) -> Self {
        Self {
            length,

            chunk_type,
            data,

            crc,
        }
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(value);
        let chunk = Self::read_chunk(&mut reader)?;

        if !reader.fill_buf()?.is_empty() {
            return Err("invalid chunk".into());
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let data = self
            .data_as_string()
            .map(|s| format!("b\"{}\"", s))
            .unwrap_or(format!("{:?}", self.data));

        write!(
            f,
            "Chunk {{ length: {}, chunk_type: b\"{}\", data: {}, crc: {} }}",
            self.length, self.chunk_type, data, self.crc
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .into();

        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), "RuSt".to_string());
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = "This is where your secret message will be!".to_string();

        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<_> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<_> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = "This is where your secret message will be!".to_string();

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), "RuSt".to_string());
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<_> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());
        assert!(chunk.is_err());
    }

    #[test]
    fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<_> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        _ = format!("{}", chunk);
    }
}
