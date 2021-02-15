use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("chunk type is not ASCII alphabetic")]
    ChunkTypeNotAsciiAlphabetic,

    #[error("invalid chunk type length {0} (expected 4)")]
    InvalidChunkTypeLength(usize),

    #[error("calculated CRC doesn't match with chunk CRC")]
    CrcMismatch,

    #[error("failed to convert chunk data to UTF-8 string")]
    NonUtf8ChunkData(#[from] std::string::FromUtf8Error),

    #[error("failed to convert slice to array")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}
