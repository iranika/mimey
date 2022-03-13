
use domain;

#[cfg(test)]
mod domain_tests {
    #[test]
    fn test_add() {
        assert_eq!(domain::add(1, 2), 3);
    }
}