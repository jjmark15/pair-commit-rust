use pair_commit_tool::models::author::{Author, AuthorVec};
use pair_commit_tool::persistence::save;

fn main() {
    println!("Hello, world!");
    let mut authors = AuthorVec::new();
    let author = Author::default();
    authors.push(author);
    save(authors);
}
