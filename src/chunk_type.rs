use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Display, Formatter};
use std::ops::BitAnd;
use std::str::FromStr;

use anyhow::{ensure, Error, Result};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ChunkType {
    ancillary: u8,
    private: u8,
    reserved: u8,
    safe_to_copy: u8,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ]
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        self.ancillary.bitand(32u8) == 0u8
    }

    fn is_public(&self) -> bool {
        self.private.bitand(32u8) == 0u8
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.reserved.bitand(32u8) == 0u8
    }

    fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy.bitand(32u8) == 32u8
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self {
            ancillary: value[0],
            private: value[1],
            reserved: value[2],
            safe_to_copy: value[3],
        })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(
            s.chars().all(|c| c.is_ascii_alphabetic()),
            "{} is not ASCII alphabetic",
            s
        );

        let bytes: [u8; 4] = s.as_bytes().try_into()?;
        Self::try_from(bytes)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.bytes()).expect("Failed to convert bytes to string")
        )
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::str::FromStr;

    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
