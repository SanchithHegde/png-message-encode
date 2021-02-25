use log::LevelFilter;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use args::{Opts, SubCommand};
use error::Error;

fn main() -> Result<(), Error> {
    use clap::Clap;

    let opts = Opts::parse();
    println!("{:#?}", opts);

    let mut builder = pretty_env_logger::formatted_timed_builder();

    if opts.verbose {
        builder.filter_level(LevelFilter::Trace);
    }

    builder.init();

    match opts.subcommand {
        SubCommand::Encode(args) => commands::encode(args),
        SubCommand::Decode(args) => commands::decode(args),
        SubCommand::Remove(args) => commands::remove(args),
        SubCommand::Print(args) => commands::print(args),
    }?;

    Ok(())
}
