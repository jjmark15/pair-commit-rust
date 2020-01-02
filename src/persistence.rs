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

#[cfg(test)]
mod tests {
    use crate::models::author::{Author, AuthorVec};
    use crate::persistence::save;

    #[test]
    fn test_write_successful() {
        let valid_path = "temp/data.yml".to_string();
        let mut authors = AuthorVec::new();
        let author = Author::default();
        authors.push(author);
        save(&valid_path, authors)
    }

    #[test]
    #[should_panic]
    fn test_write_unsuccessful() {
        let path = "temps/data.yml".to_string();
        let mut authors = AuthorVec::new();
        let author = Author::default();
        authors.push(author);
        save(&path, authors);
    }
}