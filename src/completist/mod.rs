pub mod io;
pub mod program;
pub mod formatter;
mod utils;

extern crate toml;

#[derive(Debug)]
pub enum CompletistError {
    ParserError(Vec<toml::ParserError>),
    InvalidConfig(()),
}

pub struct Completist {
    formatters: Vec<formatter::Formatter>,
}

impl Completist {
    pub fn new() -> Self {
        Completist {
            formatters: Vec::new(),
        }
    }

    pub fn add_formatter(&mut self, fmtr: formatter::Formatter) -> &mut Self {
        self.formatters.push(fmtr); self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_completist() {
        let completist = Completist::new();
        assert_eq!(completist.formatters.len(), 0);
    }

    #[test]
    fn add_formatter() {
        let mut completist = Completist::new();
        completist.add_formatter(formatter::FormatterBuilder::new("formatter").build().unwrap());
        assert_eq!(completist.formatters.len(), 1);
        completist.add_formatter(formatter::FormatterBuilder::new("formatter").build().unwrap());
        completist.add_formatter(formatter::FormatterBuilder::new("formatter").build().unwrap());
        assert_eq!(completist.formatters.len(), 3);
    }
}
