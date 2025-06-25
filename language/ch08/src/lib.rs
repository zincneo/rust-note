mod m01_match;
mod m02_pattern;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_match() {
        assert_eq!(m01_match::f01_match(), ());
        assert_eq!(m01_match::f02_if_let(), ());
        assert_eq!(m01_match::f03_while_let(), ());
        assert_eq!(m01_match::f04_matches(), ());
    }

    #[test]
    fn test02_pattern() {
        assert_eq!(m02_pattern::f01_match_pattern(), ());
        assert_eq!(m02_pattern::f02_for_pattern(), ());
        assert_eq!(m02_pattern::f03_let_pattern(), ());
        assert_eq!(m02_pattern::f04_argument_pattern(), ());
        assert_eq!(m02_pattern::f05_let_else_pattern(), ());
    }
}
