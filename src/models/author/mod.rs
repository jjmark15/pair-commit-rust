use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

pub mod author_collection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    name: String,
    email: String,
    active: bool,
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

impl Default for Author {
    fn default() -> Self {
        Author {
            name: String::from(""),
            email: String::from(""),
            active: false,
        }
    }
}

impl Author {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn new(name: String, email: String) -> Author {
        Author {
            name,
            email,
            ..Author::default()
        }
    }

    pub fn with_active_state(name: String, email: String, active: bool) -> Author {
        Author {
            name,
            email,
            active,
        }
    }

    pub fn activate(&mut self) {
        self.active = true
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn coauthor_string(&self) -> String {
        return format!("Co-authored-by: {} <{}>", self.name, self.email);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_author_initialisation() {
        let _author: Author = Author::new(String::from("Tester"), String::from("tester@test.com"));
    }

    #[test]
    fn test_with_active_state_true() {
        let author = Author::with_active_state(
            String::from("Tester"),
            String::from("tester@test.com"),
            true,
        );
        assert_eq!(true, author.active())
    }

    #[test]
    fn test_with_active_state_false() {
        let author = Author::with_active_state(
            String::from("Tester"),
            String::from("tester@test.com"),
            false,
        );
        assert_eq!(false, author.active())
    }

    #[test]
    fn test_author_active_false_default() {
        let author = Author::default();
        assert_eq!(false, author.active())
    }

    #[test]
    fn test_name() {
        let name = "Tester".to_string();
        let author = Author::new(name.clone(), "".to_string());
        assert_eq!(&name, author.name());
    }

    #[test]
    fn test_email() {
        let email = "tester@test.com".to_string();
        let author = Author::new(email.clone(), "".to_string());
        assert_eq!(&email, author.name());
    }

    #[test]
    fn test_author_activate() {
        let mut author = Author::default();
        author.activate();
        assert_eq!(true, author.active())
    }

    #[test]
    fn test_author_deactivate() {
        let mut author = Author {
            active: true,
            ..Author::default()
        };
        author.deactivate();
        assert_eq!(false, author.active());
    }

    #[test]
    fn test_author_coauthor_message() {
        let name = String::from("Tester");
        let email = String::from("tester@test.com");
        let author: Author = Author::new(name, email);
        assert_eq!(
            "Co-authored-by: Tester <tester@test.com>",
            author.coauthor_string()
        );
    }

    #[test]
    fn test_author_display() {
        let author = Author::new("Tester".to_string(), "tester@test.com".to_string());

        assert_eq!("Tester <tester@test.com>", author.to_string());
    }

    #[test]
    fn test_create_author_vec() {
        let mut authors = Vec::new();
        let author = Author::default();
        authors.push(author);
        assert_eq!(false, authors.is_empty());
    }

    #[test]
    fn test_serialize_authors() {
        let mut authors = Vec::new();
        let author = Author::new("Tester".to_string(), "tester@test.com".to_string());
        authors.push(author);

        let r = serde_yaml::to_string(&authors);
        assert_eq!(true, r.is_ok());
        let s = r.unwrap();
        let expected: String = "---\n\
                                - name: Tester\n  \
                                email: tester@test.com\n  \
                                active: false"
            .to_string();
        assert_eq!(expected, s);
    }
}