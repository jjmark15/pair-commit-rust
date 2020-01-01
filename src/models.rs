pub mod author {
    pub struct Author<'a> {
        pub name: &'a str,
        pub email: &'a str,
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_author_initialisation() {
            let _author: Author = Author { name: "Tester", email: "test@test.com" };
        }
    }
}