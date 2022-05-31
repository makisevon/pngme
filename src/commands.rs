use std::{fs, str::FromStr};

use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    Result,
};

pub fn encode(
    EncodeArgs {
        file_path,
        chunk_type,
        message,
        output_file,
    }: &EncodeArgs,
) -> Result<()> {
    let mut png: Png = fs::read(file_path)?.as_slice().try_into()?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type)?,
        message.as_bytes().into(),
    ));

    fs::write(output_file.as_ref().unwrap_or(file_path), png.as_bytes())?;
    Ok(())
}

pub fn decode(
    DecodeArgs {
        file_path,
        chunk_type,
    }: &DecodeArgs,
) -> Result<()> {
    let png: Png = fs::read(file_path)?.as_slice().try_into()?;
    match png.chunk_by_type(chunk_type) {
        Some(chunk) => println!("{}", String::from_utf8_lossy(chunk.data())),
        None => eprintln!("non-existent chunk type"),
    }

    Ok(())
}

pub fn remove(
    RemoveArgs {
        file_path,
        chunk_type,
    }: &RemoveArgs,
) -> Result<()> {
    let mut png: Png = fs::read(file_path)?.as_slice().try_into()?;
    png.remove_chunk(chunk_type)?;

    fs::write(file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(PrintArgs { file_path }: &PrintArgs) -> Result<()> {
    let png: Png = fs::read(file_path)?.as_slice().try_into()?;
    println!("{}", png);

    Ok(())
}
