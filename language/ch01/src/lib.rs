mod m01_variable;
mod m02_type;
mod m03_statement_expression;
mod m04_function;
mod m05_control_flow;
mod m06_loop;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_variable() {
        assert_eq!(m01_variable::f01_variable(), ());
        assert_eq!(m01_variable::f02_const_variable(), ());
    }

    #[test]
    fn test02_type() {
        assert_eq!(m02_type::f01_scalar_type(), ());
        assert_eq!(m02_type::f02_compound_type(), ());
    }
}
