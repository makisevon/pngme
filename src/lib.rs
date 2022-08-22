use std::{error, result};

pub mod args;
pub mod chunk;
pub mod chunk_type;
pub mod commands;
pub mod png;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;
