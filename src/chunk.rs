use crc::{Crc, CRC_32_ISO_HDLC};

use crate::{chunk_type::ChunkType, error::Error};

const CRC_32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

/// A PNG chunk.
pub(crate) struct Chunk {
    /// A 4-byte unsigned integer depicting the number of bytes in the chunk's data field.
    /// The length counts only the data field, not itself, the chunk type code, or the CRC.
    /// Zero is a valid length.
    length: u32,

    /// A 4-byte chunk type code. Must consist of uppercase or lowercase ASCII letters only.
    chunk_type: ChunkType,

    /// The data bytes appropriate to the chunk type, if any.
    /// This field can be of zero length.
    chunk_data: Vec<u8>,

    /// A 4-byte CRC calculated on the preceding bytes in the chunk, including the chunk type code
    /// and chunk data fields, but not including the length field.
    crc: u32,
}

impl Chunk {
    /// Create a new `Chunk` given the chunk type code and the chunk data.
    pub(crate) fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Self {
        let mut crc_data = chunk_type.bytes().to_vec();
        crc_data.append(&mut chunk_data.clone());
        let crc = CRC_32.checksum(&&crc_data);
        Chunk {
            length: chunk_data.len() as u32,
            chunk_type,
            chunk_data,
            crc,
        }
    }

    /// Returns the number of bytes in the chunk's data field.
    fn length(&self) -> u32 {
        self.length
    }

    /// Returns the chunk's chunk type code.
    pub(crate) fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Returns the chunk's data as a slice of bytes.
    fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    /// Returns the chunk's CRC.
    pub(crate) fn crc(&self) -> u32 {
        self.crc
    }

    /// Tries to convert the chunk's data and return it as a [`String`](String).
    pub(crate) fn data_as_string(&self) -> Result<String, Error> {
        Ok(String::from_utf8(self.chunk_data.clone())?)
    }

    /// Returns the underlying `Chunk` as a [`Vec`](Vec) of bytes.
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.length.to_be_bytes().to_vec());
        bytes.append(&mut self.chunk_type.bytes().to_vec());
        bytes.append(&mut self.chunk_data.clone());
        bytes.append(&mut self.crc.to_be_bytes().to_vec());

        bytes
    }
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        use std::convert::TryInto;

        let length = u32::from_be_bytes(value[0..4].try_into()?);
        let chunk_type: [u8; 4] = value[4..8].try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type)?;
        let chunk_data = value[8..8 + length as usize].to_vec();
        let crc = u32::from_be_bytes(value[8 + length as usize..].try_into()?);

        if CRC_32.checksum(&value[4..8 + length as usize]) != crc {
            return Err(Error::CrcMismatch);
        }

        Ok(Chunk {
            length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (Length: {}, Data: {}, CRC: {})",
            self.chunk_type,
            self.length,
            std::str::from_utf8(self.chunk_data.as_slice())
                .expect("Failed to convert chunk data to string"),
            self.crc
        )
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
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
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
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
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
