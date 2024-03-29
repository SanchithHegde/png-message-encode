use std::path::PathBuf;

use clap::{AppSettings, Parser, ValueHint};

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
pub struct Opts {
    /// Prints verbose information
    #[clap(long, short, global(true))]
    pub(crate) verbose: bool,

    #[clap(subcommand)]
    pub(crate) subcommand: SubCommand,
}

#[derive(Debug, Parser)]
pub(crate) enum SubCommand {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

/// Encode a message in a PNG file.
#[derive(Debug, Parser)]
#[clap(
    setting = AppSettings::ArgRequiredElseHelp,
)]
pub(crate) struct Encode {
    /// Path to the PNG file to encode the message in.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) in_file: PathBuf,

    /// A 4-character long ASCII alphabetic string.
    pub(crate) chunk_type: crate::chunk_type::ChunkType,

    /// Message to encode.
    pub(crate) message: String,

    /// Path to the PNG file to save the encoded image as. Optional. If this is not specified, the
    /// input PNG file is updated in place.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) out_file: Option<PathBuf>,
}

/// Decode a message in a PNG file.
#[derive(Debug, Parser)]
#[clap(
    setting = AppSettings::ArgRequiredElseHelp,
)]
pub(crate) struct Decode {
    /// Path to the PNG file to decode the message from.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) in_file: PathBuf,

    /// A 4-character long ASCII alphabetic string.
    pub(crate) chunk_type: crate::chunk_type::ChunkType,
}

/// Remove a message from a PNG file.
#[derive(Debug, Parser)]
#[clap(
    setting = AppSettings::ArgRequiredElseHelp,
)]
pub(crate) struct Remove {
    /// Path to the PNG file to remove the message from.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) in_file: PathBuf,

    /// A 4-character long ASCII alphabetic string.
    pub(crate) chunk_type: crate::chunk_type::ChunkType,
}

/// Print a list of PNG chunks that can be searched for messages
#[derive(Debug, Parser)]
#[clap(
    setting = AppSettings::ArgRequiredElseHelp,
)]
pub(crate) struct Print {
    /// Path to the PNG file to list all chunks.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) in_file: PathBuf,
}
