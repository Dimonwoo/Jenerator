use std::fs::File;
use std::io::prelude::*;

pub fn write_tokens_to_file(file_name: &str, tokens: &[String]) -> Result<usize, std::io::Error> {
    let mut file: File = File::create(file_name)?;
    for token in tokens {
        writeln!(file, "{}", token)?;
    }
    Ok(tokens.len())
}