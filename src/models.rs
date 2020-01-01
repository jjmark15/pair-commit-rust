pub mod author {
    pub struct Author {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_author() {
        assert_eq!(true, 1 == 1);
    }
}