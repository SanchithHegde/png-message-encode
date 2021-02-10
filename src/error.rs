use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("chunk type is not ASCII alphabetic")]
    ChunkTypeNotAsciiAlphabetic,

    #[error("invalid chunk type length {0} (expected 4)")]
    InvalidChunkTypeLength(usize),
}
