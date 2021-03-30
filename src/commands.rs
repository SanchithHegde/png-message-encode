use std::fs::{File, OpenOptions};

use crate::args;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::Error;
use crate::png::Png;

impl ChunkType {
    /// Returns `true` if the chunk is safe to be modified.
    /// A chunk is considered safe to be modified if it is ancillary, private, has a valid reserved
    /// bit, and is safe-to-copy.
    fn is_modifiable(&self) -> bool {
        self.is_valid()
            && !self.is_critical()
            && !self.is_public()
            && self.is_reserved_bit_valid()
            && self.is_safe_to_copy()
    }
}

pub(crate) fn encode(opts: args::Encode) -> Result<(), Error> {
    use std::{
        convert::TryFrom,
        io::{Read, Write},
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

    // Allow only safe-to-modify chunks
    if !chunk_type.is_modifiable() {
        return Err(Error::UnmodifiableChunkType(chunk_type));
    }
    png.append_chunk(Chunk::new(chunk_type, message.as_bytes().to_vec()))?;

    let mut out_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(out_file)?;
    out_file.write_all(&png.as_bytes())?;

    Ok(())
}

pub(crate) fn decode(opts: args::Decode) -> Result<(), Error> {
    use std::{convert::TryFrom, io::Read};

    let in_file = opts.in_file;
    let chunk_type = opts.chunk_type;

    // Allow only safe-to-modify chunks
    if !chunk_type.is_modifiable() {
        return Err(Error::UnmodifiableChunkType(chunk_type));
    }

    let mut in_file = File::open(in_file)?;
    let mut png_bytes = Vec::new();
    in_file.read_to_end(&mut png_bytes)?;
    let png = Png::try_from(png_bytes.as_slice())?;

    let chunk = png.chunk_by_type(&chunk_type.to_string());
    match chunk {
        Some(chunk) => {
            println!("{}", chunk.data_as_string()?);
        }
        None => return Err(Error::ChunkTypeNotFound(chunk_type)),
    }

    Ok(())
}

pub(crate) fn remove(opts: args::Remove) -> Result<(), Error> {
    use std::{
        convert::TryFrom,
        io::{Read, Write},
        str::FromStr,
    };

    let out_file = opts.in_file.clone();
    let in_file = opts.in_file;
    let chunk_type = opts.chunk_type;

    // Allow only safe-to-modify chunks
    if !chunk_type.is_modifiable() {
        return Err(Error::UnmodifiableChunkType(chunk_type));
    }

    let mut in_file = File::open(in_file)?;
    let mut png_bytes = Vec::new();
    in_file.read_to_end(&mut png_bytes)?;
    let mut png = Png::try_from(png_bytes.as_slice())?;

    png.remove_chunk(&chunk_type.to_string())?;

    let mut out_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(out_file)?;
    out_file.write_all(&png.as_bytes())?;

    Ok(())
}

pub(crate) fn print(opts: args::Print) -> Result<(), Error> {
    use std::{convert::TryFrom, io::Read};

    let in_file = opts.in_file;
    let input_file = in_file.clone();

    let mut in_file = File::open(in_file)?;
    let mut png_bytes = Vec::new();
    in_file.read_to_end(&mut png_bytes)?;
    let png = Png::try_from(png_bytes.as_slice())?;

    // Print only safe-to-modify chunks
    if !png
        .chunks()
        .iter()
        .any(|chunk| chunk.chunk_type().is_modifiable())
    {
        println!("No chunks found which could possibly contain messages");
        return Ok(());
    }

    println!("PNG chunks found in file '{}':\n", input_file.display());
    for chunk in png.chunks() {
        if chunk.chunk_type().is_modifiable() {
            println!("{}", chunk.chunk_type());
        }
    }

    Ok(())
}
