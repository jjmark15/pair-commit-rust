use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

use clap::{App, Arg, SubCommand};

use pair_commit_tool::models::author::{
    join_all_coauthor_strings, set_active_authors_in_place, Author, AuthorVec,
};

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
        let authors = load(config.save_file_path()).expect("failed");
        let output: String = get_list_command_string(&authors).unwrap_or("".to_string());
        println!("{}", output);
    } else if let Some(add_matches) = matches.subcommand_matches(CliSubCommands::Add.get_string()) {
        let mut authors = load(config.save_file_path()).expect("Failed to load existing data");
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
        authors.push(author);
        save(config.save_file_path(), &authors);
    } else if let Some(_message_matches) =
        matches.subcommand_matches(CliSubCommands::Message.get_string())
    {
        let authors = load(config.save_file_path()).expect("Failed to load existing data");
        println!("{}", join_all_coauthor_strings(&authors));
    } else if let Some(_configure_matches) =
        matches.subcommand_matches(CliSubCommands::Configure.get_string())
    {
        let mut authors = load(config.save_file_path()).expect("failed");
        let output: String = get_list_command_string(&authors).unwrap_or("".to_string());
        println!("{}", output);
        let input = get_user_input::<i32>(String::from(
            "Enter the indexes of the authors to be active",
        ));
        //        let indexes: Vec<i32> = input.iter()
        //            .filter_map(|r| r.ok().as_ref()).collect();
        //        let indexes: Vec<Result<i32, <i32 as trait>::Err>> = input.iter()
        //            .filter(|r| r.is_ok())
        //            .map(|r| r.clone().as_ref().unwrap())
        //            .collect();
        //        println!("{:?}", &input);
        //        set_active_authors_in_place(&indexes, &mut authors)
    }
}

fn get_list_command_string(authors: &AuthorVec) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(authors)
}

fn get_user_input<T: FromStr>(prompt: String) -> Vec<Result<T, T::Err>> {
    print!("{}: ", prompt);
    match read_input_line() {
        Some(s) => s
            .split(",")
            .map(|s| s.to_string().trim().parse::<T>())
            .collect(),
        None => vec![],
    }
}

fn read_input_line() -> Option<String> {
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin
        .lock()
        .read_line(&mut buf)
        .expect("Cannot read from stdin");
    if buf.trim().is_empty() {
        None
    } else {
        Some(buf.trim().to_owned())
    }
}
