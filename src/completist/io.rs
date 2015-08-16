use std::io::{stdin, Stdin, stdout, Stdout};
use std::io::{BufReader, BufWriter};
pub use std::io::{Read, Write};
use std::io::Result;
use std::fs::File;

pub type Input = BufReader<Box<Read>>;

pub fn open_input(path: &str) -> Result<Input> {
    Ok(BufReader::new(
        if path == "--" {
            Box::new(stdin())
        } else {
            Box::new(try!(File::open(path)))
        }))
}

pub type Output = BufWriter<Box<Write>>;

pub fn open_output(path: &str) -> Result<Output> {
    Ok(BufWriter::new(
        if path == "--" {
            Box::new(stdout())
        } else {
            Box::new(try!(File::create(path)))
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: work out how to test that correct inputs and outputs are opened
}
