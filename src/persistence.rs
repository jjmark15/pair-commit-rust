use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

use pair_commit_tool::models::author::{Author, AuthorCollection};

pub fn save(file_path: PathBuf, authors: &AuthorCollection) {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Problem creating the file: {:?}", error),
            _ => panic!("Unexpected problem creating the file: {:?}", error),
        },
    };

    match serde_yaml::to_writer(&file, authors.authors()) {
        Ok(()) => match file.flush() {
            Ok(s) => s,
            Err(error) => panic!("Problem writing data to file: {:?}", error),
        },
        Err(e) => panic!("Problem serializing authors to writer: {:?}", e),
    }
}

pub fn load(file_path: PathBuf) -> Result<AuthorCollection, serde_yaml::Error> {
    let file = File::open(file_path);

    match file {
        Ok(f) => {
            let vec = serde_yaml::from_reader::<File, Vec<Author>>(f);
            match vec {
                Ok(vec) => Ok(AuthorCollection::from(vec)),
                Err(e) => Err(e),
            }
        }
        Err(_) => Ok(AuthorCollection::new()),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use pair_commit_tool::models::author::{Author, AuthorCollection};

    use crate::persistence::{load, save};

    enum PersistenceFilePath {
        Basic,
        Missing,
        Writable,
        MissingParent,
    }

    impl PersistenceFilePath {
        pub fn get_filepath(&self) -> PathBuf {
            let string = match self {
                PersistenceFilePath::Basic => "test_data/persistence/basic.yml",
                PersistenceFilePath::Missing => "test_data/persistence/missing.yml",
                PersistenceFilePath::Writable => "test_data/persistence/writable.yml",
                PersistenceFilePath::MissingParent => "test_data/missing/missing.yml",
            };
            PathBuf::from(string)
        }
    }

    #[test]
    fn test_write_writable() {
        let path = PersistenceFilePath::Writable.get_filepath();
        let mut authors = AuthorCollection::new();
        let author = Author::default();
        authors.add_author(author);
        save(path, &authors)
    }

    #[test]
    #[should_panic]
    fn test_write_missing_parent() {
        let path = PersistenceFilePath::MissingParent.get_filepath();
        let mut authors = AuthorCollection::new();
        let author = Author::default();
        authors.add_author(author);
        save(path, &authors);
    }

    #[test]
    fn test_load_missing() {
        let path = PersistenceFilePath::Missing.get_filepath();
        let data = load(path);
        assert_eq!(true, data.is_ok());
    }

    #[test]
    fn test_load_existing() {
        let path = PersistenceFilePath::Basic.get_filepath();
        let data = load(path);
        assert_eq!(true, data.is_ok());
        let authors = data.unwrap();
        assert_eq!(false, authors.authors().is_empty())
    }
}
