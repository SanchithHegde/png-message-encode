use std::path::PathBuf;

use clap::{AppSettings, Clap, ValueHint};

#[derive(Clap, Debug)]
#[clap(
    author,
    version,
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
pub(crate) enum Opt {
    /// Encode a message in a PNG file.
    #[clap(
        setting = AppSettings::ColoredHelp,
        setting = AppSettings::ArgRequiredElseHelp,
    )]
    Encode {
        /// Path to the PNG file to encode the message in.
        #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
        in_file: PathBuf,

        /// A 4-character long alphabetical ASCII string.
        chunk_type: String,

        /// Message to encode.
        message: String,

        /// Path to the PNG file to save the encoded image as. Optional. If this is not specified,
        /// the input PNG file is updated in place.
        #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
        out_file: Option<PathBuf>,
    },

    /// Decode a message in a PNG file.
    #[clap(
        setting = AppSettings::ColoredHelp,
        setting = AppSettings::ArgRequiredElseHelp,
    )]
    Decode {
        /// Path to the PNG file to decode the message from.
        #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
        in_file: PathBuf,

        /// A 4-character long alphabetical ASCII string.
        chunk_type: String,
    },

    /// Remove a message from a PNG file.
    #[clap(
        setting = AppSettings::ColoredHelp,
        setting = AppSettings::ArgRequiredElseHelp,
    )]
    Remove {
        /// Path to the PNG file to remove the message from.
        #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
        in_file: PathBuf,

        /// A 4-character long alphabetical ASCII string.
        chunk_type: String,
    },

    /// Print all messages in a PNG file. This could generate text which is NOT human-readable.
    #[clap(
        setting = AppSettings::ColoredHelp,
        setting = AppSettings::ArgRequiredElseHelp,
    )]
    Print {
        /// Path to the PNG file to print all the messages.
        #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
        in_file: PathBuf,
    },
}
