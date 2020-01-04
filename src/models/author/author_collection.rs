use std::convert::TryFrom;

use crate::models::author::Author;

pub struct AuthorCollection {
    authors: Vec<Author>,
}

impl AuthorCollection {
    pub fn new() -> AuthorCollection {
        AuthorCollection {
            ..AuthorCollection::default()
        }
    }

    fn from_vec(vec: Vec<Author>) -> AuthorCollection {
        AuthorCollection { authors: vec }
    }

    pub fn add_author(&mut self, author: Author) {
        self.authors_mut().push(author);
    }

    pub fn authors(&self) -> &Vec<Author> {
        &self.authors
    }

    pub fn authors_mut(&mut self) -> &mut Vec<Author> {
        &mut self.authors
    }

    pub fn active_authors(&self) -> Vec<&Author> {
        self.authors.iter().filter(|a| a.active()).collect()
    }

    pub fn set_active_authors_by_indexes(&mut self, indexes: &[i32]) {
        for (index, author) in self.authors.iter_mut().enumerate() {
            let i32_index: i32 = i32::try_from(index).expect("failed to convert usize to i32");
            if indexes.contains(&i32_index) {
                author.activate()
            } else {
                author.deactivate()
            }
        }
    }

    pub fn join_all_active_coauthor_strings(&self) -> String {
        self.active_authors()
            .iter()
            .map(|author| author.coauthor_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn authors_with_indexes(&self) -> String {
        self.authors()
            .iter()
            .enumerate()
            .map(|(index, author)| {
                format!(
                    "- index: {}\n  name: {}\n  email: {}",
                    index,
                    author.name(),
                    author.email()
                )
            })
            .collect::<Vec<String>>()
            .join("\n---\n")
    }
}

impl Default for AuthorCollection {
    fn default() -> Self {
        AuthorCollection { authors: vec![] }
    }
}

impl From<Vec<Author>> for AuthorCollection {
    fn from(vec: Vec<Author>) -> Self {
        AuthorCollection::from_vec(vec)
    }
}

impl Into<Vec<Author>> for AuthorCollection {
    fn into(self) -> Vec<Author> {
        self.authors
    }
}

#[cfg(test)]
mod tests {
    use crate::models::author::author_collection::AuthorCollection;
    use crate::models::author::Author;

    #[test]
    fn test_join_all_active_coauthor_strings() {
        let authors = AuthorCollection::from(vec![
            Author::with_active_state("Tester".to_string(), "tester@test.com".to_string(), true),
            Author::with_active_state("Tester".to_string(), "tester@test.com".to_string(), true),
        ]);
        assert_eq!(
            "Co-authored-by: Tester <tester@test.com>\n\
             Co-authored-by: Tester <tester@test.com>",
            authors.join_all_active_coauthor_strings()
        );
    }

    #[test]
    fn test_set_active_authors_by_indexes() {
        let mut authors = AuthorCollection::from(vec![
            Author::new("Tester".to_string(), "tester@test.com".to_string()),
            Author::with_active_state("Tester".to_string(), "tester@test.com".to_string(), true),
        ]);

        authors.set_active_authors_by_indexes(&[0 as i32]);
        assert!(authors.authors().get(0).unwrap().active);
        assert!(!authors.authors().get(1).unwrap().active);
    }

    #[test]
    fn test_add_author() {
        let mut authors = AuthorCollection::new();
        authors.add_author(Author::default());
        assert!(!authors.authors().is_empty());
    }

    #[test]
    fn test_active_authors() {
        let mut authors = AuthorCollection::new();
        authors.add_author(Author::default());
        authors.add_author(Author::with_active_state(
            "Tester".to_string(),
            "tester@test.com".to_string(),
            true,
        ));
        let active = authors.active_authors();
        assert_eq!(1, active.len());
    }

    #[test]
    fn test_from_vec() {
        let collection: AuthorCollection = AuthorCollection::from(vec![Author::default()]);
        assert_eq!(1, collection.authors().len());
    }

    #[test]
    fn test_authors_with_indexes() {
        let authors = AuthorCollection::from(vec![
            Author::new("Tester".to_string(), "tester@test.com".to_string()),
            Author::with_active_state("Tester".to_string(), "tester@test.com".to_string(), true),
        ]);
        let s = authors.authors_with_indexes();
        let expected = "- index: 0\n  \
                        name: Tester\n  \
                        email: tester@test.com\n\
                        ---\n\
                        - index: 1\n  \
                        name: Tester\n  \
                        email: tester@test.com"
            .to_string();
        assert_eq!(expected, s);
    }
}
