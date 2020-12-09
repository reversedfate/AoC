#[cfg(test)]
mod tests {
    #[test]
    fn basic_equals() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn basic_not_equals() {
        assert_ne!(2 + 2, 5);
    }
}