mod m01_struct;
mod m02_enum;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_struct() {
        assert_eq!(m01_struct::f01_struct(), ());
        assert_eq!(m01_struct::f02_memory(), ());
        assert_eq!(m01_struct::f03_tuple_struct(), ());
    }

    #[test]
    fn test02_enum() {
        assert_eq!(m02_enum::f01_enum(), ());
        assert_eq!(m02_enum::f02_option(), ());
        assert_eq!(m02_enum::f03_result(), ());
    }
}
