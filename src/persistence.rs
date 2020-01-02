use std::fs::File;
use std::io::{ErrorKind, Write};

use crate::models::author::AuthorVec;

pub fn save(file_path: &String, authors: AuthorVec) {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Problem creating the file: {:?}", error),
            _ => panic!("Unexpected problem creating the file: {:?}", error),
        }
    };

    match serde_yaml::to_writer(&file, &authors) {
        Ok(()) => match file.flush() {
            Ok(s) => s,
            Err(error) => panic!("Problem writing data to file: {:?}", error)
        },
        Err(e) => panic!("Problem serializing authors to writer: {:?}", e)
    }
}

pub fn load(file_path: &String) -> Result<AuthorVec, serde_yaml::Error> {
    let file = File::open(file_path);

    match file {
        Ok(f) => {
            serde_yaml::from_reader::<File, AuthorVec>(f)
        }
        Err(_) => Ok(AuthorVec::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::author::{Author, AuthorVec};
    use crate::persistence::{load, save};

    enum PersistenceFilePath {
        Basic,
        Missing,
        Writable,
        MissingParent,
    }

    impl PersistenceFilePath {
        pub fn get_filepath(&self) -> &str {
            match self {
                PersistenceFilePath::Basic => "test_data/persistence/basic.yml",
                PersistenceFilePath::Missing => "test_data/persistence/missing.yml",
                PersistenceFilePath::Writable => "test_data/persistence/writable.yml",
                PersistenceFilePath::MissingParent => "test_data/missing/missing.yml"
            }
        }
    }

    #[test]
    fn test_write_writable() {
        let valid_path = PersistenceFilePath::Writable.get_filepath().to_string();
        let mut authors = AuthorVec::new();
        let author = Author::default();
        authors.push(author);
        save(&valid_path, authors)
    }

    #[test]
    #[should_panic]
    fn test_write_missing_parent() {
        let path = PersistenceFilePath::MissingParent.get_filepath().to_string();
        let mut authors = AuthorVec::new();
        let author = Author::default();
        authors.push(author);
        save(&path, authors);
    }

    #[test]
    fn test_load_missing() {
        let path = PersistenceFilePath::Missing.get_filepath().to_string();
        let data = load(&path);
        assert_eq!(true, data.is_ok());
    }

    #[test]
    fn test_load_existing() {
        let path = PersistenceFilePath::Basic.get_filepath().to_string();
        let data = load(&path);
        assert_eq!(true, data.is_ok());
        let authors = data.unwrap();
        assert_eq!(false, authors.is_empty())
    }
}