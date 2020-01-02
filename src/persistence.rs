use std::fs::File;
use std::io;
use std::io::{ErrorKind, Write};

use crate::models::author::AuthorVec;

pub fn save(authors: AuthorVec) -> io::Result<()> {
    let mut file = match File::create("temp/data.yml") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Problem creating the file: {:?}", error),
            _ => panic!("Problem creating the file: {:?}", error),
        }
    };

    match serde_yaml::to_writer(&file, &authors) {
        Ok(()) => file.flush(),
        Err(e) => panic!("Problem opening the file: {:?}", e)
    }
}