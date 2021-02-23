use std::path::PathBuf;

use clap::{AppSettings, Clap, ValueHint};

#[derive(Clap, Debug)]
#[clap(
    author,
    version,
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
pub(crate) enum SubCommand {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

/// Encode a message in a PNG file.
#[derive(Clap, Debug)]
#[clap(
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::ArgRequiredElseHelp,
)]
pub(crate) struct Encode {
    /// Path to the PNG file to encode the message in.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) in_file: PathBuf,

    /// A 4-character long alphabetical ASCII string.
    pub(crate) chunk_type: String,

    /// Message to encode.
    pub(crate) message: String,

    /// Path to the PNG file to save the encoded image as. Optional. If this is not specified, the
    /// input PNG file is updated in place.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) out_file: Option<PathBuf>,
}

/// Decode a message in a PNG file.
#[derive(Clap, Debug)]
#[clap(
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::ArgRequiredElseHelp,
)]
pub(crate) struct Decode {
    /// Path to the PNG file to decode the message from.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) in_file: PathBuf,

    /// A 4-character long alphabetical ASCII string.
    pub(crate) chunk_type: String,
}

/// Remove a message from a PNG file.
#[derive(Clap, Debug)]
#[clap(
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::ArgRequiredElseHelp,
)]
pub(crate) struct Remove {
    /// Path to the PNG file to remove the message from.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) in_file: PathBuf,

    /// A 4-character long alphabetical ASCII string.
    pub(crate) chunk_type: String,
}

/// Print a list of PNG chunks that can be searched for messages
#[derive(Clap, Debug)]
#[clap(
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::ArgRequiredElseHelp,
)]
pub(crate) struct Print {
    /// Path to the PNG file to list all chunks.
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub(crate) in_file: PathBuf,
}
