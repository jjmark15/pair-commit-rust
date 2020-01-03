use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

use pair_commit_tool::models::author::Author;

pub fn get_list_command_string(authors: &[Author]) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(authors)
}

pub fn get_user_input<T: FromStr>(prompt: String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    print!("{}: ", prompt);
    let string: String = read_input_line();
    if string.is_empty() {
        Vec::new()
    } else {
        split_string_to_vec::<T>(string)
    }
}

fn split_string_to_vec<T: FromStr>(s: String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<T>>()
}

fn read_input_line() -> String {
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin
        .lock()
        .read_line(&mut buf)
        .expect("Cannot read from stdin");
    buf.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use crate::cli::user_input::*;

    #[test]
    fn test_split_string_to_vec_i32() {
        let string = "1 2 3 4".to_string();
        let vec = split_string_to_vec::<i32>(string);
        assert_eq!(vec![1, 2, 3, 4], vec);
    }
}
