use std::fmt::Display;
use std::path::PathBuf;
use std::{env, error};

use serde::export::Formatter;

#[derive(Debug)]
pub struct Config {
    app_home: Option<PathBuf>,
    save_file_name: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app_home: None,
            save_file_name: "data.yml",
        }
    }
}

impl Config {
    fn get_new_app_home() -> Result<PathBuf, Error> {
        match env::var("PAIR_COMMIT_HOME") {
            Ok(s) => Ok(PathBuf::from(s)),
            Err(_e) => get_default_app_home(),
        }
    }

    pub fn new() -> Result<Config, Error> {
        match Config::get_new_app_home() {
            Ok(home) => Ok(Config {
                app_home: Option::from(home),
                ..Config::default()
            }),
            Err(e) => Err(e),
        }
    }

    pub fn save_file_path(&self) -> Option<PathBuf> {
        match &self.app_home {
            Some(_h) => {
                let mut path = PathBuf::new();
                path.push(&self.app_home.as_ref().unwrap_or(&PathBuf::new()));
                path.push(&self.save_file_name);
                Some(path)
            }
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Error {
        Error { message }
    }
}

impl Default for Error {
    fn default() -> Self {
        Error {
            message: "A config error occurred".to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {}

fn get_default_app_home() -> Result<PathBuf, Error> {
    const APP_DIR_NAME: &str = ".pair_commit_tool";
    match dirs::home_dir() {
        Some(path) => {
            let mut home = path;
            home.push(APP_DIR_NAME);
            Ok(home)
        }
        None => Err(Error::new(
            "Failed to get default home directory".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::config::{Config, Error};

    #[test]
    fn test_save_file_path() {
        let config = Config::new().unwrap();
        let re = Regex::new(r"/home/\w*/.pair_commit_tool").unwrap();
        assert_eq!(
            true,
            re.is_match(config.save_file_path().unwrap().to_str().unwrap())
        );
    }

    #[test]
    fn test_config_error() {
        let _error: Result<String, Error> = Ok("test".to_string());
    }
}
