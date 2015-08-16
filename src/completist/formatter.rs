use std::collections::HashSet;
use std::io::Error;

use completist::utils::normalise_extension;
use completist::io::{Output, Write};
use completist::program::{Program, Command, Argument, Opt};

pub struct Formatter {
    pub name: String,
    extensions: HashSet<String>,
}

pub type FmtResult = Result<(), Error>;

impl Formatter {
    pub fn matches_extension(&self, extension: String) -> bool {
        self.extensions.contains(&extension)
    }

    pub fn write_header(&self, program: &Program, output: &mut Output) -> FmtResult {
        // Here I should write the subcommand function to allow anyone to use it
        Ok(())
    }

    pub fn write_opt(&self, option: &Opt, output: &mut Output) -> FmtResult {
        try!(output.write_fmt(format_args!("complete -c '{}' ", self.name)));
        for short in &option.shorts {
            try!(output.write_fmt(format_args!("-s '{}'", short)));
        }

        for long in &option.longs {
            if long.starts_with("--") {
                try!(output.write_fmt(format_args!("-l '{}'", long)));
            } else {  // of form "-option"
                try!(output.write_fmt(format_args!("-o '{}'", long)));
            }
        }

        try!(output.write_fmt(format_args!("-d '{}'", option.description)));

        Ok(())
    }

    pub fn write_arg(&self, option: &Opt, output: &mut Output) -> FmtResult {
        Ok(())
    }
}

pub struct FormatterBuilder {
    name: String,
    extensions: HashSet<String>,
}

impl FormatterBuilder {
    pub fn new(name: &str) -> Self {
        FormatterBuilder {
            name: name.to_string(),
            extensions: HashSet::new(),
        }
    }

    pub fn ext(&mut self, ext: &str) -> &mut Self {
        self.extensions.insert(normalise_extension(ext.to_string()));
        self
    }

    pub fn exts(&mut self, exts: &[&str]) -> &mut Self {
        for ext in exts { self.ext(ext); }
        self
    }

    pub fn build(self) -> Result<Formatter, ()> {
        Ok(Formatter {
            name: self.name,
            extensions: self.extensions,
        })
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    mod builder {
        use super::FormatterBuilder;

        #[test]
        fn construct_builder() {
            let builder = FormatterBuilder::new("formatter name");
            assert_eq!(builder.name, "formatter name");
            let formatter = builder.build().ok().expect("Not all required params filled in");
            assert_eq!(formatter.name, "formatter name");
        }

        #[test]
        fn update_extensions() {
            let mut builder = FormatterBuilder::new("formatter name");
            builder.ext(".fish").ext(".fsh").ext(".fish-completion");
            assert_eq!(builder.extensions.len(), 3);

            builder.ext("fish");  // should be treated the same as ".fish"
            assert_eq!(builder.extensions.len(), 3);

            builder.exts(&[".fs", ".fishy", ".fish.completion"]);
            assert_eq!(builder.extensions.len(), 6);
        }

        #[test]
        fn build_extensions() {
            let mut builder = FormatterBuilder::new("formatter name");
            builder.ext(".fish").exts(&[".fsh", "fish-completion"]);
            let formatter = builder.build().ok().expect("Not all required params filled in");
            assert_eq!(formatter.extensions.len(), 3);

            let builder = FormatterBuilder::new("formatter name");
            let formatter = builder.build().ok().expect("Not all required params filled in");
            assert_eq!(formatter.extensions.len(), 0);
        }
    }

    mod formatter {
        use super::*;

        #[test]
        fn matches_extension() {
            let mut builder = FormatterBuilder::new("formatter");
            builder.exts(&[".fish", ".fsh"]);
            let formatter = builder.build().ok().expect("Not all required params filled in");

            assert!(formatter.matches_extension(".fish".to_string()));
            assert!(!formatter.matches_extension(".fhs".to_string()));
        }
    }
}
