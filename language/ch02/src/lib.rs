mod m01_ownership;
mod m02_reference;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_ownership() {
        assert_eq!(m01_ownership::f01_ownership(), ());
        assert_eq!(m01_ownership::f02_data_bind(), ());
    }

    #[test]
    fn test02_reference() {
        assert_eq!(m02_reference::f01_ref(), ());
        assert_eq!(m02_reference::f02_mut_ref(), ());
    }
}
