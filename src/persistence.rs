use std::fs::File;
use std::io::{ErrorKind, Write};

use crate::models::author::AuthorVec;

pub fn save(authors: AuthorVec) {
    let mut file = match File::create("temp/data.yml") {
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