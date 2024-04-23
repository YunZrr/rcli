use anyhow::Result;
use std::{fs::File, io::Read};

pub fn get_buf(input: &str) -> Result<String> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    Ok(buf.trim().to_owned())
}
