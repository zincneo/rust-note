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
        assert_eq!(m01_variable::f03_mutability(), ());
        assert_eq!(m01_variable::f04_shoadwing(), ());
    }

    #[test]
    fn test02_type() {
        assert_eq!(m02_type::f01_scalar_type(), ());
        assert_eq!(m02_type::f02_compound_type(), ());
    }

    #[test]
    fn test03_statement_expression() {
        assert_eq!(m03_statement_expression::f01_statement_expression(), ());
    }

    #[test]
    fn test04_function() {
        assert_eq!(m04_function::f01_function(), ());
        assert_eq!(m04_function::f02_divergence_function(), ());
    }

    #[test]
    fn test05_control_flow() {
        assert_eq!(m05_control_flow::f01_if_else(), ());
        assert_eq!(m05_control_flow::f02_match(), ());
    }
}
