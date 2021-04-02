use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("invalid chunk type '{0}' (expected a 4-character ASCII alphabetic string)")]
    InvalidChunkType(String),

    #[error(
        "invalid chunk type '{0}' (expected a chunk type which is ancillary, private, has a valid \
        reserved bit and is safe-to-copy)"
    )]
    UnmodifiableChunkType(crate::chunk_type::ChunkType),

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

    #[error("IEND chunk not found in PNG file. Is the PNG file corrupted?")]
    IendChunkNotFound,

    #[error("chunk of type {0} already exists in file!")]
    ChunkTypeExists(crate::chunk_type::ChunkType),
}
