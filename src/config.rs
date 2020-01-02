use std::env;

#[derive(Debug)]
pub struct Config {
    app_home: String,
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
    fn get_new_app_home() -> String {
        const DEFAULT: &str = "~/.pair_commit_tool";
        match env::var("PAIR_COMMIT_HOME") {
            Ok(s) => s,
            Err(_e) => DEFAULT.to_string()
        }
    }

    pub fn new() -> Config {
        Config {
            ..Config::default()
        }
    }

    pub fn save_file_path(&self) -> String {
        format!("{}/{}", &self.app_home, &self.save_file_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_save_file_path() {
        let config = Config::new();
        assert_eq!("~/.pair_commit_tool/data.yml", config.save_file_path())
    }
}