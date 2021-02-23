use thiserror::Error;

#[allow(clippy::enum_variant_names)]
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

    #[error("chunk of type {0} not found")]
    ChunkTypeNotFound(crate::chunk_type::ChunkType),

    #[error("invalid PNG file size {0} (expected at least 8)")]
    InvalidPngFileSize(usize),

    #[error("PNG file header doesn't match with standard PNG header")]
    PngHeaderMismatch,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
