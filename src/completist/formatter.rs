use std::collections::HashSet;
use completist::utils::normalise_extension;

pub struct Formatter {
    pub name: String,
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
        for ext in exts {
            self.ext(ext);
        }
        self
    }

    pub fn build(self) -> Result<Formatter, ()> {
        Ok(Formatter { name: self.name })
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

    }
}
