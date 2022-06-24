use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        self.bytes.iter().all(u8::is_ascii_alphabetic) && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0] >> 5 & 1 == 0
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1] >> 5 & 1 == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] >> 5 & 1 == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] >> 5 & 1 == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        if !value.iter().all(u8::is_ascii_alphabetic) {
            return Err("invalid chunk type".into());
        }

        Ok(Self { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        <[_; 4]>::try_from(s.as_bytes())?.try_into()
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2 = FromStr::from_str("RuSt").unwrap();

        _ = format!("{}", chunk_type_1);
        _ = chunk_type_1 == chunk_type_2;
    }
}
