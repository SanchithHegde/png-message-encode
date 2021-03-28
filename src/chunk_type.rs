use crate::error::Error;

/// A 4-byte chunk type code. Must consist of uppercase or lowercase ASCII letters only.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ChunkType {
    /// Four bits of the type code, namely bit 5 (value 32) of each byte, are used to convey chunk
    /// properties.
    /// The assigned properties can be determined by testing whether each letter of the type code
    /// is uppercase (bit 5 is 0) or lowercase (bit 5 is 1).
    ///
    /// For more information, check the [PNG File Structure Specification] page.
    ///
    /// [PNG File Structure Specification]: http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-naming-conventions

    /// # Ancillary bit: bit 5 of first byte
    /// 0 (uppercase) = critical, 1 (lowercase) = ancillary.
    ///
    /// Chunks that are not strictly necessary in order to meaningfully display the contents of the
    /// file are known as "ancillary" chunks.
    /// Chunks that are necessary for successful display of the file's contents are called
    /// "critical" chunks.
    ancillary: u8,

    /// # Private bit: bit 5 of second byte
    /// 0 (uppercase) = public, 1 (lowercase) = private.
    ///
    /// A public chunk is one that is part of the PNG specification or is registered in the list of
    /// PNG special-purpose public chunk types.
    private: u8,

    /// # Reserved bit: bit 5 of third byte
    /// Must be 0 (uppercase) in files conforming to version 1.2 of the PNG specification.
    reserved: u8,

    /// # Safe-to-copy bit: bit 5 of fourth byte
    /// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
    ///
    /// If a chunk's safe-to-copy bit is 0, it indicates that the chunk depends on the image data.
    safe_to_copy: u8,
}

impl ChunkType {
    /// Returns the underlying fields as a byte array.
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ]
    }

    /// Returns `true` if the chunk is considered valid.
    /// A valid chunk must have all characters in the type code to be ASCII alphabetic and the
    /// reserved bit must be valid.
    fn is_valid(&self) -> bool {
        self.bytes().iter().all(|&c| c.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }

    /// Returns `true` if the chunk is a critical chunk.
    fn is_critical(&self) -> bool {
        use std::ops::BitAnd;

        self.ancillary.bitand(32u8) == 0u8
    }

    /// Returns `true` if the chunk is a public chunk.
    fn is_public(&self) -> bool {
        use std::ops::BitAnd;

        self.private.bitand(32u8) == 0u8
    }

    /// Returns `true` if the reserved bit is set to zero.
    fn is_reserved_bit_valid(&self) -> bool {
        use std::ops::BitAnd;

        self.reserved.bitand(32u8) == 0u8
    }

    /// Returns `true` if the chunk's safe-to-copy bit is 1, i.e., the chunk does not depend on the
    /// image data.
    fn is_safe_to_copy(&self) -> bool {
        use std::ops::BitAnd;

        self.safe_to_copy.bitand(32u8) == 32u8
    }
}

impl std::convert::TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !value.iter().all(|c| c.is_ascii_alphabetic()) {
            return Err(Error::InvalidChunkType(
                std::str::from_utf8(&value).unwrap().to_string(),
            ));
        }

        Ok(Self {
            ancillary: value[0],
            private: value[1],
            reserved: value[2],
            safe_to_copy: value[3],
        })
    }
}

impl std::str::FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 || !s.as_bytes().iter().all(|c| c.is_ascii_alphabetic()) {
            return Err(Error::InvalidChunkType(s.to_string()));
        }

        let s = s.as_bytes();
        Ok(Self {
            ancillary: s[0],
            private: s[1],
            reserved: s[2],
            safe_to_copy: s[3],
        })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(&self.bytes()) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(std::fmt::Error),
        }
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
