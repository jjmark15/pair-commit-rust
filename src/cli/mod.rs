use std::path::PathBuf;

use clap::{App, Arg, SubCommand};

use pair_commit_tool::models::author::{join_all_coauthor_strings, Author, AuthorCollection};

use crate::cli::user_input::{get_list_command_string, get_user_input};
use crate::config::Config;
use crate::persistence::{load, save};

mod user_input;

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
            CliSubCommands::Message => "message",
        }
    }
}

pub fn init() {
    let config = Config::new();
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name(CliSubCommands::List.get_string())
                .about("Lists all co-authors with their metadata"),
        )
        .subcommand(
            SubCommand::with_name(CliSubCommands::Add.get_string())
                .about("Add a new co-author")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .multiple(false)
                        .takes_value(true)
                        .value_name("NAME")
                        .help("Set new co-author name"),
                )
                .arg(
                    Arg::with_name("email")
                        .short("e")
                        .long("email")
                        .required(true)
                        .multiple(false)
                        .takes_value(true)
                        .value_name("EMAIL")
                        .help("Set new co-author email"),
                )
                .arg(
                    Arg::with_name("active")
                        .short("a")
                        .long("active")
                        .required(false)
                        .multiple(false)
                        .takes_value(false)
                        .help("Set new co-author as active"),
                ),
        )
        .subcommand(
            SubCommand::with_name(CliSubCommands::Configure.get_string())
                .about("Configure which co-authors are active"),
        )
        .subcommand(
            SubCommand::with_name(CliSubCommands::Message.get_string())
                .about("Get a co-authors message to append to a git commit"),
        )
        .get_matches();

    if let Some(_list_matches) = matches.subcommand_matches(CliSubCommands::List.get_string()) {
        let authors = load(config.save_file_path()).expect("Failed to load existing data");
        handle_list_sub_command(authors);
    } else if let Some(add_matches) = matches.subcommand_matches(CliSubCommands::Add.get_string()) {
        let authors = load(config.save_file_path()).expect("Failed to load existing data");
        let author = Author::with_active_state(
            add_matches
                .value_of("name")
                .expect("Name value not found")
                .to_string(),
            add_matches
                .value_of("email")
                .expect("Email value not found")
                .to_string(),
            add_matches.is_present("active"),
        );
        handle_add_sub_command(authors, author, config.save_file_path());
    } else if let Some(_message_matches) =
        matches.subcommand_matches(CliSubCommands::Message.get_string())
    {
        let authors = load(config.save_file_path()).expect("Failed to load existing data");
        handle_message_sub_command(authors);
    } else if let Some(_configure_matches) =
        matches.subcommand_matches(CliSubCommands::Configure.get_string())
    {
        let authors = load(config.save_file_path()).expect("failed");
        handle_configure_sub_command(authors, config.save_file_path());
    }
}

fn handle_list_sub_command(author_col: AuthorCollection) {
    let output: String = get_list_command_string(&author_col).unwrap_or_else(|_| "".to_string());
    println!("{}", output);
}

fn handle_add_sub_command(mut authors: AuthorCollection, new_author: Author, file_path: PathBuf) {
    authors.add_author(new_author);
    save(file_path, &authors);
}

fn handle_message_sub_command(authors: AuthorCollection) {
    println!("{}", join_all_coauthor_strings(&authors.authors()));
}

fn handle_configure_sub_command(mut authors: AuthorCollection, file_path: PathBuf) {
    let output: String = get_list_command_string(&authors).unwrap_or_else(|_| "".to_string());
    println!("{}", output);
    let indexes = get_user_input::<i32>(String::from(
        "Enter the indexes of the authors to be active",
    ));
    authors.set_active_authors_by_indexes(&indexes);
    save(file_path, &authors);
}
