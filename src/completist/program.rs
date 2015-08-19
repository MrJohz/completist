use std::ascii::AsciiExt;
extern crate regex;
extern crate shlex;
extern crate toml;

pub type CompResult<'a, T> = Result<T, String>;

#[derive(Debug, PartialEq, Eq)]
pub enum ArgKind {
    File,
    FilePlus,
    Anything,
    Separator,
    OneOf(Vec<String>),
    Command(String),
    Function(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Opt {
    opts: Vec<String>,
    argument: Option<ArgKind>,
}

impl Opt {
    fn new(opts: Vec<String>, argument: Option<ArgKind>) -> Self {
        Opt {
            opts: opts,
            argument: argument,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Arg {
    kind: ArgKind,
    required: bool,
    usefiles: bool,
}

impl Arg {
    fn new(kind: ArgKind, required: bool, usefiles: bool) -> Self {
        Arg {
            kind: kind,
            required: required,
            usefiles: usefiles,
        }
    }

    fn capture_regex<F>(inp: &str, re: regex::Regex, former: F) -> Option<ArgKind>
        where F : Fn(String) -> Option<ArgKind> {

        if let Some(capture) = re.captures(inp).and_then(|i| i.at(1)) {
            former(capture
                .replace(r"\(", r"(")
                .replace(r"\)", r")")
                .replace(r"\\", r"\"))
        } else {
            None
        }
    }

    fn parse_argkind(inp: &str) -> CompResult<ArgKind> {
        let fnre = regex::Regex::new(r"(?xs)
            ^(?i:function)\(
                ( (?: [^\\)(] | \\\( | \\\) | \\\\ )* )
            \)$").unwrap();
        let fn_conv = |x: String| {
            Some(ArgKind::Function(x))
        };
        let cmdre = regex::Regex::new(r"(?xs)
            ^(?i:command)\(
                ( (?: [^\\)(] | \\\( | \\\) | \\\\ )* )
            \)$").unwrap();
        let cmd_conv = |x: String| {
            Some(ArgKind::Command(x))
        };
        let oneofre = regex::Regex::new(r"(?xs)
            ^(?i:oneof|one_of)\(
                ( (?: [^\\)(] | \\\( | \\\) | \\\\ )* )
            \)$").unwrap();
        let oneof_conv = |x: String| {
            shlex::split(&*x).map(|i| ArgKind::OneOf(i))
        };

        if inp.eq_ignore_ascii_case("file") {
            Ok(ArgKind::File)
        } else if inp.eq_ignore_ascii_case("file+") {
            Ok(ArgKind::FilePlus)
        } else if inp.eq_ignore_ascii_case("separator") {
            Ok(ArgKind::Separator)
        } else if inp.eq_ignore_ascii_case("anything") {
            Ok(ArgKind::Anything)
        } else if let Some(capture) = Self::capture_regex(inp, fnre, fn_conv) {
            Ok(capture)
        } else if let Some(capture) = Self::capture_regex(inp, cmdre, cmd_conv) {
            Ok(capture)
        } else if let Some(capture) = Self::capture_regex(inp, oneofre, oneof_conv) {
            Ok(capture)
        } else {
            Err(format!("Unrecognised arg kind {}", inp))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Subcmd {
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompKind {
    Opt(Opt),
    Arg(Arg),
    Subcmd(Subcmd),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Completion {
    command: String,
    path: Vec<String>,
    completion_kind: CompKind,
    description: String,
}

impl Completion {
    fn new(cmd: String, path: Vec<String>, kind: CompKind, desc: String) -> Self {
        Completion {
            command: cmd,
            path: path,
            completion_kind: kind,
            description: desc,
        }
    }

    fn build_arg(cmd: String, path: Vec<String>, toml: &toml::Table)
        -> CompResult<Self> {

        let kind = try!(toml
            .get("kind")
            .ok_or(format!(
                "missing required option 'kind' (path: {p:?})", p = path))
            .and_then(|kind| kind.as_str().ok_or(format!(
                "value for 'kind' (path: {p:?}) must be of type str: {k:?}",
                p = path, k = kind)))
            .and_then(|kind| Arg::parse_argkind(kind)));
        let required = match toml.get("required") {
            Some(required) =>
                try!(required
                    .as_bool()
                    .ok_or(format!(
                        "value for 'required' (path: {p:?}) must be of type bool: {r:?}",
                        p = path, r = required))),
            None => false,
        };
        let usefiles = match toml.get("usefiles") {
            Some(usefiles) =>
                try!(usefiles
                    .as_bool()
                    .ok_or(format!(
                        "value for 'usefiles' (path: {p:?}) must be of type bool: {u:?}",
                        p = path, u = usefiles))),
            None => true,
        };

        let completion_kind = CompKind::Arg(Arg::new(kind, required, usefiles));
        let desc = try!(toml
            .get("description")
            .ok_or(format!(
                "option missing required key 'description' (path: {p:?})",
                p = path))
            .and_then(|desc| desc.as_str().ok_or(format!(
                "value for 'description' (path: {p:?}) must be str: {d:?}",
                p = path, d = desc)))
            .map(|desc| desc.to_string()));

        Ok(Self::new(cmd, path, completion_kind, desc))
    }

    fn build_opt(cmd: String, path: Vec<String>, toml: &toml::Table)
        -> CompResult<Self> {

        let mut opts = Vec::new();

        if let Some(o) = toml.get("opt") {
            opts.push(try!(o.as_str()
                .map(|o| o.to_string())
                .ok_or(format!(
                    "value for 'opt' (path: {p:?}) must be of type str: {o:?}",
                    p = path, o = o))));
        };

        if let Some(os) = toml.get("opts") {
            let os = try!(os.as_slice()
                .ok_or(format!(
                    "value for 'opts' (path: {p:?}) must be of type list: {os:?}",
                    p=path, os=os)));
            for opt in os {
                opts.push(try!(opt.as_str()
                    .map(|o| o.to_string())
                    .ok_or(format!(
                        "value for 'opts' (path: {p:?}) must be list of str: {os:?}",
                        p = path, os = os))));
            }
        }

        let argument =
            match toml.get("argument") {
                Some(arg) =>
                    Some(try!(arg
                        .as_str()
                        .ok_or(format!(
                            "value for 'argument' (path: {p:?}) must be str: {a:?}",
                            p = path, a = arg))
                        .and_then(|arg| Arg::parse_argkind(arg)))),
                None =>
                    None,
            };

        let completion_kind = CompKind::Opt(Opt::new(opts, argument));
        let desc = try!(toml
            .get("description")
            .ok_or(format!(
                "option missing required key 'description' (path: {p:?})",
                p = path))
            .and_then(|desc| desc.as_str().ok_or(format!(
                "value for 'description' (path: {p:?}) must be str: {d:?}",
                p = path, d = desc)))
            .map(|desc| desc.to_string()));

        Ok(Self::new(cmd, path, completion_kind, desc))
    }

    pub fn from_toml(toml: &toml::Table) -> CompResult<Vec<Self>> {
        let mut result = Vec::new();

        for (key, value) in toml.iter() {
            let value = try!(value
                .as_table()
                .ok_or(format!(
                    "command {k:?} must map to a table structure, not: {t:?}",
                    k = key, t = value)));
            if let Some(name) = key.split(".").next() {
                let name = name;
                let subcmds: Vec<String> = key
                    .split(".").map(|x| x.to_string()).collect();

                if let Some(options) = value.get("option") {
                    let options = try!(options
                        .as_slice()
                        .ok_or(format!(
                            "use [[double-bracketed]] lists to define option at {k:?}",
                            k = key)));
                    for option in options {
                        let option: CompResult<Self> = option.as_table()
                            .ok_or(format!("options must be table-structures"))
                            .and_then(|o| {
                                Self::build_opt(name.to_string(), subcmds.clone(), o)
                            });
                        match option {
                            Ok(option) => result.push(option),
                            Err(err) => return Err(err),
                        }
                    }
                }

                if let Some(args) = value.get("argument") {
                    let args = try!(args
                        .as_slice()
                        .ok_or(format!(
                            "use [[double bracketed]] lists to defined argument at {k:?}",
                            k = key)));
                    for argument in args {
                        let argument: CompResult<Self> = argument.as_table()
                            .ok_or(format!("arguments must be table-structures"))
                            .and_then(|a| {
                                Self::build_arg(name.to_string(), subcmds.clone(), a)
                            });
                        match argument {
                            Ok(argument) => result.push(argument),
                            Err(err) => return Err(err),
                        }
                    }
                }

            } else {
                return Err(format!("Invalid command key: {}", key));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    extern crate toml;

    #[test]
    fn completion_opt_from_toml() {
        let toml = toml::Parser::new("
            [[prog.option]]
            opt = '--lib'
            description = 'option'").parse().expect("TOML could not be parsed");
        let completions = Completion::from_toml(&toml).unwrap();
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].command, "prog".to_string());
        assert_eq!(completions[0].path, vec!["prog".to_string()]);
        assert_eq!(completions[0].description, "option".to_string());
        if let CompKind::Opt(ref opt) = completions[0].completion_kind {
            assert_eq!(opt.opts, vec!["--lib".to_string()]);
            assert_eq!(opt.argument, None);
        } else {
            panic!("Wrong CompKind returned");
        }
    }

    #[test]
    fn completion_arg_from_toml() {
        let toml = toml::Parser::new("
            [[prog.argument]]
            kind = 'FILE'
            description = 'arg'").parse().expect("TOML could not be parsed");
        let completions = Completion::from_toml(&toml).unwrap();
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].command, "prog".to_string());
        assert_eq!(completions[0].path, vec!["prog".to_string()]);
        assert_eq!(completions[0].description, "arg".to_string());
        if let CompKind::Arg(ref arg) = completions[0].completion_kind {
            assert_eq!(arg.required, false);
            assert_eq!(arg.usefiles, true);
            assert_eq!(arg.kind, ArgKind::File);
        } else {
            panic!("Wrong CompKind returned");
        }
    }

    #[test]
    fn parse_argkind() {
        let kind = Arg::parse_argkind("FILE").unwrap();
        assert_eq!(kind, ArgKind::File);
        let kind = Arg::parse_argkind("FiLe").unwrap();
        assert_eq!(kind, ArgKind::File);
        let kind = Arg::parse_argkind("FILE+").unwrap();
        assert_eq!(kind, ArgKind::FilePlus);
        let kind = Arg::parse_argkind("anything").unwrap();
        assert_eq!(kind, ArgKind::Anything);
        let kind = Arg::parse_argkind("separator").unwrap();
        assert_eq!(kind, ArgKind::Separator);
        let kind = Arg::parse_argkind("oneof(a b c)").unwrap();
        assert_eq!(kind, ArgKind::OneOf(
            vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        let kind = Arg::parse_argkind("one_of(a b c)").unwrap();
        assert_eq!(kind, ArgKind::OneOf(
            vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        let kind = Arg::parse_argkind(r#"one_of('a' "b" '')"#).unwrap();
        assert_eq!(kind, ArgKind::OneOf(
            vec!["a".to_string(), "b".to_string(), "".to_string()]));
        let kind = Arg::parse_argkind("on_e_of(a b c d)");
        assert!(kind.is_err());
        let kind = Arg::parse_argkind("function(a b c d)").unwrap();
        assert_eq!(kind, ArgKind::Function("a b c d".to_string()));
        let kind = Arg::parse_argkind("command(a b c d)").unwrap();
        assert_eq!(kind, ArgKind::Command("a b c d".to_string()));
        let kind = Arg::parse_argkind("unrecognised input");
        assert!(kind.is_err());
    }
}
