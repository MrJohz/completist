use std::ascii::AsciiExt;

extern crate toml;
extern crate regex;

#[derive(Debug, PartialEq, Eq)]
pub enum OptKind {
    File,
    FilePlus,
    OneOf(String),
    Command(String),
    Function(String),
}

pub fn construct_optkind(inp: &str) -> Option<OptKind> {
    let fnre = regex::Regex::new(r"(?xs)
        ^(?i:function)\(
            ( (?: [^\\)(] | \\\( | \\\) | \\\\ )* )
        \)$").unwrap();
    let cmdre = regex::Regex::new(r"(?xs)
        ^(?i:command)\(
            ( (?: [^\\)(] | \\\( | \\\) | \\\\ )* )
        \)$").unwrap();
    let oneofre = regex::Regex::new(r"(?xs)
        ^(?i:oneof|one_of)\(
            ( (?: [^\\)(] | \\\( | \\\) | \\\\ )* )
        \)$").unwrap();

    if inp.eq_ignore_ascii_case("file") {
        Some(OptKind::File)
    } else if inp.eq_ignore_ascii_case("file+") {
        Some(OptKind::FilePlus)
    } else if let Some(capture) = cmdre.captures(inp).and_then(|i| i.at(1)) {
        Some(OptKind::Command(capture
            .replace("\\(", "(")
            .replace("\\)", ")")
            .replace("\\\\", "\\")
            .to_string()))
    } else if let Some(capture) = fnre.captures(inp).and_then(|i| i.at(1)) {
        Some(OptKind::Function(capture
            .replace("\\(", "(")
            .replace("\\)", ")")
            .replace("\\\\", "\\")
            .to_string()))
    } else if let Some(capture) = oneofre.captures(inp).and_then(|i| i.at(1)) {
        Some(OptKind::OneOf(capture
            .replace("\\(", "(")
            .replace("\\)", ")")
            .replace("\\\\", "\\")
            .to_string()))
    } else {
        None
    }
}

pub struct Command {
    name: String,
    arguments: Vec<Argument>,
    options: Vec<Opt>,
    commands: Vec<Command>,
}

impl Command {
    fn new(name: &str) -> Self {
        Command {
            name: name.to_string(),
            arguments: Vec::new(),
            options: Vec::new(),
            commands: Vec::new(),
        }
    }

    fn from_toml(data: &toml::Table) -> Option<Self> {
        let name = match data.get("name").and_then(|name| name.as_str()) {
            Some(name) => name,
            None => return None,
        };

        let mut command = Self::new(name);

        if let Some(arguments) = data.get("argument").and_then(|a| a.as_slice()) {
            for argument in arguments {
                match argument.as_table().and_then(|t| Argument::from_toml(t)) {
                    Some(argument) => command.arguments.push(argument),
                    None => continue,
                }
            }
        }

        if let Some(options) = data.get("option").and_then(|a| a.as_slice()) {
            for option in options {
                match option.as_table().and_then(|t| Opt::from_toml(t)) {
                    Some(option) => command.options.push(option),
                    None => continue,
                }
            }
        }

        if let Some(commands) = data.get("command").and_then(|a| a.as_slice()) {
            for subcommand in commands {
                match subcommand.as_table().and_then(|t| Command::from_toml(t)) {
                    Some(subcommand) => command.commands.push(subcommand),
                    None => continue,
                }
            }
        }

        Some(command)
    }
}

pub struct Argument {
    name: String,
    kind: OptKind,
    optional: bool,
}

impl Argument {
    fn new(name: &str, kind: OptKind, optional: bool) -> Self {
        Argument {
            name: name.to_string(),
            kind: kind,
            optional: optional,
        }
    }

    fn from_toml(table: &toml::Table) -> Option<Self> {
        let name = table.get("name")
            .and_then(|a| a.as_str());
        let kind = table.get("kind")
            .and_then(|a| a.as_str())
            .and_then(|a| construct_optkind(a));
        let optional = table.get("optional")
            .and_then(|a| a.as_bool());
        if name.is_some() && kind.is_some() {
            Some(Argument::new(name.unwrap(), kind.unwrap(), optional.unwrap_or(false)))
        } else {
            None
        }
    }
}

pub struct Opt {
    pub longs: Vec<String>,
    pub shorts: Vec<String>,
    pub description: String,
    pub argkind: Option<OptKind>,
}

impl Opt {
    fn new(longs: Vec<String>, shorts: Vec<String>,
            description: &str, argkind: Option<OptKind>) -> Self {
        Opt {
            longs: longs,
            shorts: shorts,
            description: description.to_string(),
            argkind: argkind,
        }
    }

    fn normalize_long(s: String) -> String {
        if s.starts_with("-") {
            s
        } else {
            format!("--{}", s)
        }
    }

    fn normalize_short(s: String) -> String {
        if s.starts_with("-") {
            s
        } else {
            format!("-{}", s)
        }
    }

    fn from_toml(table: &toml::Table) -> Option<Self> {
        let description = table.get("description")
            .and_then(|a| a.as_str());
        let argkind = table.get("argkind")
            .and_then(|a| a.as_str())
            .and_then(|a| construct_optkind(a));
        let mut long_vec = Vec::new();
        if let Some(long) = table.get("long").and_then(|a| a.as_str()) {
            long_vec.push(Self::normalize_long(long.to_string()));
        } else if let Some(longs) = table.get("longs").and_then(|a| a.as_slice()) {
            for long in longs {
                if let Some(long) = long.as_str() {
                    long_vec.push(Self::normalize_long(long.to_string()));
                }
            }
        }
        let mut short_vec = Vec::new();
        if let Some(short) = table.get("short").and_then(|a| a.as_str()) {
            short_vec.push(Self::normalize_short(short.to_string()));
        } else if let Some(shorts) = table.get("shorts").and_then(|a| a.as_slice()) {
            for short in shorts {
                if let Some(short) = short.as_str() {
                    short_vec.push(Self::normalize_short(short.to_string()));
                }
            }
        }

        if description.is_some() && (short_vec.len() > 0 || long_vec.len() > 0) {
            Some(Self::new(long_vec, short_vec, description.unwrap(), argkind))
        } else {
            None
        }
    }
}

pub struct Program {
    pub name: String,
    base_command: Command,
}

impl Program {
    fn new(name: &str, base_command: Command) -> Self {
        Program {
            name: name.to_string(),
            base_command: base_command,
        }
    }

    pub fn from_toml(data: &toml::Table) -> Result<Self, ()> {
        let base_command = try!(Command::from_toml(data).ok_or(()));

        let name = try!(data.get("name").ok_or(()));
        let name = try!(name.as_str().ok_or(()));
        let prog = Self::new(name, base_command);
        Ok(prog)
    }
}

#[cfg(test)]
mod tests {
    extern crate toml;
    use super::*;

    #[test]
    fn create_program() {
        let toml = toml::Parser::new("name = 'test-command'").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();

        assert_eq!(prog.name, "test-command");
        assert_eq!(prog.base_command.name, "test-command");
    }

    #[test]
    fn create_program_with_arguments() {
        let toml = toml::Parser::new("
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'file+'
            optional = true
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();

        assert_eq!(prog.base_command.arguments.len(), 1);
        assert_eq!(prog.base_command.arguments[0].name, "FILE");
        assert_eq!(prog.base_command.arguments[0].kind, OptKind::FilePlus);
        assert_eq!(prog.base_command.arguments[0].optional, true);

        let toml = toml::Parser::new("
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'file+'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();

        assert_eq!(prog.base_command.arguments.len(), 1);
        assert_eq!(prog.base_command.arguments[0].name, "FILE");
        assert_eq!(prog.base_command.arguments[0].kind, OptKind::FilePlus);
        assert_eq!(prog.base_command.arguments[0].optional, false);
    }

    #[test]
    fn different_argkinds() {
        let toml = toml::Parser::new("
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'fILe'  # should be case-insensitive
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();
        assert_eq!(prog.base_command.arguments.len(), 1);
        assert_eq!(prog.base_command.arguments[0].kind, OptKind::File);

        let toml = toml::Parser::new("
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'file+'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();
        assert_eq!(prog.base_command.arguments.len(), 1);
        assert_eq!(prog.base_command.arguments[0].kind, OptKind::FilePlus);

        let toml = toml::Parser::new(r"
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'function(can contain \( and \) inside)'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();
        assert_eq!(prog.base_command.arguments.len(), 1);
        assert_eq!(prog.base_command.arguments[0].kind,
            OptKind::Function("can contain ( and ) inside".to_string()));

        let toml = toml::Parser::new(r"
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'command(can contain escaped \\ inside)'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();
        assert_eq!(prog.base_command.arguments.len(), 1);
        assert_eq!(prog.base_command.arguments[0].kind,
            OptKind::Command("can contain escaped \\ inside".to_string()));

        let toml = toml::Parser::new(r"
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'command(escaping \\\( \\\\ rules \\)'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();
        assert_eq!(prog.base_command.arguments.len(), 1);
        assert_eq!(prog.base_command.arguments[0].kind,
            OptKind::Command("escaping \\( \\\\ rules \\".to_string()));

        let toml = toml::Parser::new(r"
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'command(err: unescaped \ inside)'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();
        assert_eq!(prog.base_command.arguments.len(), 0);

        let toml = toml::Parser::new(r"
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'function(err: unescaped ( & ) inside)'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();
        assert_eq!(prog.base_command.arguments.len(), 0);

        let toml = toml::Parser::new(r"
            name = 'test-command'
            [[argument]]
            name = 'FILE'
            kind = 'unrecognised kind'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();
        assert_eq!(prog.base_command.arguments.len(), 0);
    }

    #[test]
    fn create_program_with_options() {
        let toml = toml::Parser::new("
            name = 'test-command'
            [[option]]
            long = '--option'
            short = '-o'
            description = 'desc'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();

        assert_eq!(prog.base_command.options.len(), 1);
        assert_eq!(prog.base_command.options[0].longs.len(), 1);
        assert_eq!(prog.base_command.options[0].longs[0], "--option");
        assert_eq!(prog.base_command.options[0].shorts.len(), 1);
        assert_eq!(prog.base_command.options[0].shorts[0], "-o");
        assert_eq!(prog.base_command.options[0].description, "desc");
        assert_eq!(prog.base_command.options[0].argkind, None);

        let toml = toml::Parser::new("
            name = 'test-command'
            [[option]]
            longs = ['--option']
            shorts = ['-o']
            argkind = 'FILE'
            description = 'desc'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();

        assert_eq!(prog.base_command.options.len(), 1);
        assert_eq!(prog.base_command.options[0].longs.len(), 1);
        assert_eq!(prog.base_command.options[0].longs[0], "--option");
        assert_eq!(prog.base_command.options[0].shorts.len(), 1);
        assert_eq!(prog.base_command.options[0].shorts[0], "-o");
        assert_eq!(prog.base_command.options[0].description, "desc");
        assert_eq!(prog.base_command.options[0].argkind, Some(OptKind::File));
    }

    #[test]
    fn normalise_options() {
        let toml = toml::Parser::new("
            name = 'test-command'
            [[option]]
            long = 'option'
            short = 'o'
            description = 'implicit long/short option'

            [[option]]
            long = '--opt'
            short = '-o'
            description = 'normal long/short option'

            [[option]]
            long = '-myoption'
            description = 'old-style long option'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();

        assert_eq!(prog.base_command.options.len(), 3);
        assert_eq!(prog.base_command.options[0].longs[0], "--option");
        assert_eq!(prog.base_command.options[0].shorts[0], "-o");
        assert_eq!(prog.base_command.options[1].longs[0], "--opt");
        assert_eq!(prog.base_command.options[1].shorts[0], "-o");
        assert_eq!(prog.base_command.options[2].longs[0], "-myoption");
    }

    #[test]
    fn create_program_with_subcommands() {
        let toml = toml::Parser::new("
            name = 'test-command'
            [[command]]
            name = 'sub-command'
            [[command.argument]]
            name = 'FILE'
            kind = 'FILE+'
            [[command.option]]
            longs = ['--option']
            description = 'desc'
        ").parse().unwrap();
        let prog = Program::from_toml(&toml).unwrap();

        assert_eq!(prog.base_command.commands.len(), 1);
        assert_eq!(prog.base_command.commands[0].arguments.len(), 1);
        assert_eq!(prog.base_command.commands[0].options.len(), 1);
    }
}
