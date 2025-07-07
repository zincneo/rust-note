mod m01_match;
mod m02_pattern;
mod m03_scence;
mod m04_deconstruction;

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

    #[test]
    fn test03_scence() {
        assert_eq!(m03_scence::f01_range_pattern(), ());
        assert_eq!(m03_scence::f02_composition_pattern(), ());
        assert_eq!(m03_scence::f03_if_guard_pattern(), ());
        assert_eq!(m03_scence::f04_at_pattern(), ());
    }

    #[test]
    fn test04_deconstruction() {
        assert_eq!(m04_deconstruction::f01_tuple(), ());
        assert_eq!(m04_deconstruction::f02_struct(), ());
        assert_eq!(m04_deconstruction::f03_array(), ());
        assert_eq!(m04_deconstruction::f04_enum(), ());
    }
}
