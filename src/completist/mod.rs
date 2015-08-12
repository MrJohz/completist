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
    programs: Vec<program::Program>,
    formatters: Vec<formatter::Formatter>,
}

impl Completist {
    pub fn new() -> Self {
        Completist {
            programs: Vec::new(),
            formatters: Vec::new(),
        }
    }

    pub fn add_formatter(&mut self, fmtr: formatter::Formatter) -> &mut Self {
        self.formatters.push(fmtr); self
    }

    pub fn parse_string(&mut self, string: &str)
                        -> Result<&mut Self, CompletistError> {
        let mut parser = toml::Parser::new(string);
        let toml = try!(parser.parse()
            .ok_or_else(move || parser.errors)
            .map_err(CompletistError::ParserError));

        let prog = try!(program::Program::from_toml(&toml)
            .map_err(CompletistError::InvalidConfig));

        self.programs.push(prog);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_completist() {
        let completist = Completist::new();
        assert_eq!(completist.programs.len(), 0);
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

    #[test]
    fn parse_string() {
        let mut completist = Completist::new();
        completist.parse_string("name = 'test-command'").unwrap();
        assert_eq!(completist.programs.len(), 1);
        completist.parse_string("name = 'another-command'").unwrap();
        completist.parse_string("name = 'command-three'").unwrap();
        assert_eq!(completist.programs.len(), 3);
    }
}
