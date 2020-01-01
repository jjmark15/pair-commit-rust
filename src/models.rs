pub mod author {
    pub struct Author {
        name: String,
        email: String,
        active: bool,
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
            return self.active;
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn email(&self) -> &String {
            &self.email
        }

        pub fn new(name: String, email: String) -> Author {
            return Author {
                name,
                email,
                ..Author::default()
            };
        }

        pub fn activate(&mut self) {
            self.active = true
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
            let _author: Author = Author::new(
                String::from("Tester"), String::from("tester@test.com"));
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
        fn test_author_coauthor_message() {
            let name = String::from("Tester");
            let email = String::from("tester@test.com");
            let author: Author = Author::new(name, email);
            assert_eq!("Co-authored-by: Tester <tester@test.com>", author.coauthor_string());
        }
    }
}