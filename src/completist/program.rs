use std::ascii::AsciiExt;
extern crate regex;
extern crate shlex;

type CompResult<'a, T> = Result<T, String>;

#[derive(Debug, PartialEq, Eq)]
pub enum ArgKind {
    File,
    FilePlus,
    OneOf(Vec<String>),
    Command(String),
    Function(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Opt {
    longs: Vec<String>,
    shorts: Vec<String>,
    argument: Arg,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Arg {
    name: String,
    kind: ArgKind,
    required: bool,
    exclusive: bool,
}

impl Arg {
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
    subcommand: Vec<String>,
    completion_kind: CompKind,
    description: String,
}

impl Completion {
    fn new(cmd: String, path: Vec<String>, kind: CompKind, desc: String) -> Self {
        Completion {
            command: cmd,
            subcommand: path,
            completion_kind: kind,
            description: desc,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parse_argkind() {
        let kind = Arg::parse_argkind("FILE").unwrap();
        assert_eq!(kind, ArgKind::File);
        let kind = Arg::parse_argkind("FiLe").unwrap();
        assert_eq!(kind, ArgKind::File);
        let kind = Arg::parse_argkind("FILE+").unwrap();
        assert_eq!(kind, ArgKind::FilePlus);
        let kind = Arg::parse_argkind("oneof(a b c)").unwrap();
        assert_eq!(kind, ArgKind::OneOf(
            vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        let kind = Arg::parse_argkind("one_of(a b c)").unwrap();
        assert_eq!(kind, ArgKind::OneOf(
            vec!["a".to_string(), "b".to_string(), "c".to_string()]));
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
