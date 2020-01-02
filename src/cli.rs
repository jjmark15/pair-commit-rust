use clap::{App, SubCommand};

use pair_commit_tool::models::author::Author;

use crate::config::Config;
use crate::persistence::{load, save};

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
    let config = Config::new();
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
        let authors = load(&config.save_file_path()).expect("failed");
        println!("{:?}", authors);
    }

    if let Some(_add_matches) = matches.subcommand_matches(CliSubCommands::Add.get_string()) {
        let mut authors = load(&config.save_file_path())
            .expect("Failed to load existing data");
        let author = Author::new(
            "Josh".to_string(),
            "j@j.com".to_string(),
        );
        authors.push(author);
        save(&config.save_file_path(), &authors);
    }
}