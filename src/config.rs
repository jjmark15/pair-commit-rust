use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    app_home: PathBuf,
    save_file_name: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app_home: Config::get_new_app_home(),
            save_file_name: "data.yml",
        }
    }
}

impl Config {
    fn get_new_app_home() -> PathBuf {
        match env::var("PAIR_COMMIT_HOME") {
            Ok(s) => PathBuf::from(s),
            Err(_e) => get_default_app_home(),
        }
    }

    pub fn new() -> Config {
        Config {
            ..Config::default()
        }
    }

    pub fn save_file_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(&self.app_home);
        path.push(&self.save_file_name);
        path
    }
}

fn get_default_app_home() -> PathBuf {
    const APP_DIR_NAME: &str = ".pair_commit_tool";
    let user_home = dirs::home_dir().expect("Could not determine user home directory");
    let mut default = user_home;
    default.push(APP_DIR_NAME);
    default
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::config::Config;

    #[test]
    fn test_save_file_path() {
        let config = Config::new();
        let re = Regex::new(r"/home/\w*/.pair_commit_tool").unwrap();
        assert_eq!(true, re.is_match(config.save_file_path().to_str().unwrap()));
    }
}
