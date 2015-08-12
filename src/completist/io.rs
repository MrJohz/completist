use std::io::{stdin, Stdin, stdout, Stdout};
use std::io::{BufReader, BufWriter};
use std::io::Result;
use std::fs::File;

pub enum Input {
    StndIn(BufReader<Stdin>),
    FileIn(BufReader<File>),
}

impl Input {
    pub fn open(path: &str) -> Result<Self> {
        if path == "--" {
            Ok(Input::StndIn(BufReader::new(stdin())))
        } else {
            Ok(Input::FileIn(BufReader::new(try!(File::open(path)))))
        }
    }
}

pub enum Output {
    StndOut(BufWriter<Stdout>),
    FileOut(BufWriter<File>),
}

impl Output {
    pub fn open(path: &str) -> Result<Self> {
        if path == "--" {
            Ok(Output::StndOut(BufWriter::new(stdout())))
        } else {
            Ok(Output::FileOut(BufWriter::new(try!(File::create(path)))))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate tempdir;

    #[test]
    fn input_opens_correctly() {
        let inp = Input::open("--").ok().expect("should always be able to open stdin");
        match inp {
            Input::StndIn(_) => assert!(true),
            Input::FileIn(_) => assert!(false),
        }

        let tmpdir = tempdir::TempDir::new("input_opens_correctly").unwrap();
        let inp = Input::open(tmpdir.path().join("nonexistant").to_str().unwrap());
        assert!(inp.is_err());
    }

    #[test]
    fn output_opens_correctly() {
        let outp = Output::open("--")
            .ok().expect("should always be able to open stdin");
        match outp {
            Output::StndOut(_) => assert!(true),
            Output::FileOut(_) => assert!(false),
        }

        let tmpdir = tempdir::TempDir::new("output_opens_correctly").unwrap();
        let outp = Output::open(tmpdir.path().join("file.txt").to_str().unwrap())
            .ok().expect("should always be able to open new file");
        match outp {
            Output::StndOut(_) => assert!(false),
            Output::FileOut(_) => assert!(true),
        }
    }
}
