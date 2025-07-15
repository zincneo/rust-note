mod m01_drop;
mod m02_deref;
mod m03_operator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_drop() {
        assert_eq!(m01_drop::f01_drop_function(), ());
        assert_eq!(m01_drop::f02_drop_trait(), ());
    }

    #[test]
    fn test02_deref() {
        assert_eq!(m02_deref::f01_ref_arg(), ());
        assert_eq!(m02_deref::f02_deref_trait(), ());
    }

    #[test]
    fn test03_operator() {
        assert_eq!(m03_operator::f01_add(), ());
    }
}
