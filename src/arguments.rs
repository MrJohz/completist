struct Command {
    subcommands: Vec<Command>,
    arguments: Vec<Argument>,
    options: Vec<Opt>
}
