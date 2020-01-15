use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

use pair_commit_tool::models::author::author_collection::AuthorCollection;

pub fn get_list_command_string(author_col: &AuthorCollection) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(author_col.authors())
}

pub fn get_user_input<P: AsRef<str>, T: FromStr + Default>(prompt: P) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    print!("{}: ", prompt.as_ref());
    let string: String = read_input_line();
    if string.is_empty() {
        Vec::new()
    } else {
        split_string_to_vec(string, ",".parse().unwrap())
    }
}

fn split_string_to_vec<P: AsRef<str>, T: FromStr + Default>(s: P, split_string: char) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.as_ref()
        .split(split_string)
        .map(|s| s.trim().parse().unwrap_or_default())
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
    fn test_split_string_to_vec_i32_space() {
        let string = "1 2 3 4".to_string();
        let vec: Vec<i32> = split_string_to_vec(string, " ".parse().unwrap());
        assert_eq!(vec![1, 2, 3, 4], vec);
    }

    #[test]
    fn test_split_string_to_vec_i32_comma() {
        let string = "1, 2, 3, 4";
        let vec: Vec<i32> = split_string_to_vec(string, ",".parse().unwrap());
        assert_eq!(vec![1, 2, 3, 4], vec);
    }
}
