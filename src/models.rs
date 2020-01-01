pub mod author {
    pub struct Author<'a> {
        pub name: &'a str,
        pub email: &'a str,
        active: bool,
    }

    impl Default for Author<'_> {
        fn default() -> Self {
            Author {
                name: "",
                email: "",
                active: false,
            }
        }
    }

    impl Author<'_> {
        pub fn is_active(&self) -> bool {
            return self.active;
        }

        pub fn new<'a>(name: &'a str, email: &'a str) -> Author<'a> {
            return Author {
                name,
                email,
                ..Author::default()
            };
        }

        pub fn activate(&mut self) {
            self.active = true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_author_initialisation() {
            let _author: Author = Author::new("Tester", "tester@test.com");
        }

        #[test]
        fn test_author_active_false_default() {
            let author = Author::default();
            assert_eq!(false, author.is_active())
        }

        #[test]
        fn test_author_activate() {
            let mut author = Author::default();
            author.activate();
            assert_eq!(true, author.is_active())
        }
    }
}