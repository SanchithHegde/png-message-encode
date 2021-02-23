use std::fs::{File, OpenOptions};

use crate::args;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::Error;
use crate::png::Png;

pub(crate) fn encode(opts: args::Encode) -> Result<(), Error> {
    use std::{
        convert::TryFrom,
        io::{Read, Write},
        str::FromStr,
    };

    let out_file = if opts.out_file.is_none() {
        opts.in_file.clone()
    } else {
        opts.out_file.unwrap()
    };
    let in_file = opts.in_file;
    let chunk_type = opts.chunk_type;
    let message = opts.message;

    let mut in_file = File::open(in_file)?;
    let mut png_bytes = Vec::new();
    in_file.read_to_end(&mut png_bytes)?;
    let mut png = Png::try_from(png_bytes.as_slice())?;

    png.append_chunk(Chunk::new(
        ChunkType::from_str(&chunk_type)?,
        message.as_bytes().to_vec(),
    ));

    let mut out_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(out_file)?;
    out_file.write_all(&png.as_bytes())?;

    Ok(())
}
