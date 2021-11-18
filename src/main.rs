#![warn(clippy::pedantic)]

use log::LevelFilter;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use args::{Opts, SubCommand};

fn main() {
    use clap::Parser;

    let opts = Opts::parse();
    let mut builder = pretty_env_logger::formatted_timed_builder();

    if opts.verbose {
        builder.filter_level(LevelFilter::Trace);
    }

    builder.init();

    if let Err(error) = match opts.subcommand {
        SubCommand::Encode(args) => commands::encode(args),
        SubCommand::Decode(args) => commands::decode(args),
        SubCommand::Remove(args) => commands::remove(args),
        SubCommand::Print(args) => commands::print(args),
    } {
        log::error!("{}", error);
    }
}
