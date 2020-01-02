use clap::{App, SubCommand};

enum CliSubCommands {
    List,
    Add,
    Configure,
    Message,
}

impl CliSubCommands {
    pub fn get_string(&self) -> &str {
        match self {
            CliSubCommands::List => "list",
            CliSubCommands::Add => "add",
            CliSubCommands::Configure => "configure",
            CliSubCommands::Message => "message"
        }
    }
}

pub fn init() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name(CliSubCommands::List.get_string())
            .about("Lists all co-authors with their metadata"))
        .subcommand(SubCommand::with_name(CliSubCommands::Add.get_string())
            .about("Add a new co-author"))
        .subcommand(SubCommand::with_name(CliSubCommands::Configure.get_string())
            .about("Configure which co-authors are active"))
        .subcommand(SubCommand::with_name(CliSubCommands::Message.get_string())
            .about("Get a co-authors message to append to a git commit"))
        .get_matches();

    if let Some(_list_matches) = matches.subcommand_matches(CliSubCommands::List.get_string()) {
        println!("list");
    }
}