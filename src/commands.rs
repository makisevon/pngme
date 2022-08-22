use std::{fs, str::FromStr};

use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    Result,
};

pub fn encode(args: &EncodeArgs) -> Result<()> {
    let mut png: Png = fs::read(&args.file_path)?.as_slice().try_into()?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.as_bytes().into(),
    ));

    fs::write(
        args.output_file.as_ref().unwrap_or(&args.file_path),
        png.as_bytes(),
    )?;

    Ok(())
}

pub fn decode(args: &DecodeArgs) -> Result<()> {
    let png: Png = fs::read(&args.file_path)?.as_slice().try_into()?;
    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => println!("{}", String::from_utf8_lossy(chunk.data())),
        None => eprintln!("non-existent chunk type"),
    }

    Ok(())
}

pub fn remove(args: &RemoveArgs) -> Result<()> {
    let mut png: Png = fs::read(&args.file_path)?.as_slice().try_into()?;
    png.remove_chunk(&args.chunk_type)?;

    fs::write(&args.file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(args: &PrintArgs) -> Result<()> {
    let png: Png = fs::read(&args.file_path)?.as_slice().try_into()?;
    println!("{}", png);

    Ok(())
}
