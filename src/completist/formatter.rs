use std::collections::HashSet;
use std::io::Error;

use completist::utils::normalise_extension;
use completist::io::{Output, Write};
use completist::program::{Program, Command, Argument, Opt, OptKind};

pub struct Formatter {
    pub name: String,
    extensions: HashSet<String>,
}

pub type FmtResult = Result<(), Error>;

impl Formatter {
    pub fn matches_extension(&self, extension: String) -> bool {
        self.extensions.contains(&extension)
    }

    pub fn write_comment(&self, output: &mut Output, text: String) -> FmtResult {
        for line in text.lines() {
            try!(output.write_fmt(format_args!("# {}", line)));
        }

        Ok(())
    }

    pub fn write_header(&self, output: &mut Output, program: &Program) -> FmtResult {
        try!(output.write_fmt(format_args!(r#"
            function __fish_at_level_{}
              set cmd (commandline -opc)
              set subcommands $argv
              set subcommands_len (count $subcommands)
              if [ (count $cmd) -eq $subcommands_len ]
                for i in (seq $subcommands_len)
                  if [ $subcommands[$i] != $cmd[$i] ]
                    return 1
                  end
                end
                return 0
              end

              return 1
            end\n"#, program.name)));
        Ok(())
    }

    pub fn write_begin(&self, out: &mut Output, prog: &Program) -> FmtResult {
        try!(out.write_fmt(format_args!("complete -c '{}' ", prog.name)));
        Ok(())
    }

    pub fn write_level(&self, out: &mut Output, prog: &Program, lvl: &[String]) -> FmtResult {
        try!(out.write_fmt(format_args!(" -n '__fish_at_level_{} {}' ",
            prog.name, lvl.connect(" "))));
        Ok(())
    }

    pub fn write_opt(&self, out: &mut Output, opt: &Opt) -> FmtResult {
        for short in &opt.shorts {
            try!(out.write_fmt(format_args!(" -s '{}' ", short)));
        }

        for long in &opt.longs {
            try!(out.write_fmt(format_args!(" -{} '{}' ",
                (if long.starts_with("--") {"l"} else {"o"}), long)));
        }

        Ok(())
    }

    pub fn write_opt_description(&self, out: &mut Output, opt: &Opt) -> FmtResult {
        try!(out.write_fmt(format_args!(" -d '{}' ", opt.description)));
        Ok(())
    }

    pub fn write_opt_arguments(&self, out: &mut Output, opt: &Opt) -> FmtResult {
        if let Some(ref argkind) = opt.argkind {
            match argkind {
                &OptKind::File =>
                    try!(out.write_fmt(format_args!(" -r "))),
                &OptKind::FilePlus =>
                    try!(out.write_fmt(format_args!(" -r -a '--'"))),
                &OptKind::OneOf(ref args) =>
                    try!(out.write_fmt(
                        format_args!(" -r -a '{}'", args.replace(r"'", r"\'")))),
                &OptKind::Command(ref cmd) =>
                    try!(out.write_fmt(format_args!(" -r -a '({})'", cmd))),
                &OptKind::Function(ref func) =>
                    try!(out.write_fmt(
                        format_args!(" -x -a '(__fish_completist_func_{})", func)))
            }
        }
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
