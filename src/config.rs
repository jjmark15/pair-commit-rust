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
        let default = PathBuf::from("~/.pair_commit_tool");
        match env::var("PAIR_COMMIT_HOME") {
            Ok(s) => PathBuf::from(s),
            Err(_e) => default
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
        return path;
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::Config;

    #[test]
    fn test_save_file_path() {
        let config = Config::new();
        println!("{:?}", config.save_file_path());
        assert_eq!(PathBuf::from("~/.pair_commit_tool/data.yml"), config.save_file_path())
    }
}