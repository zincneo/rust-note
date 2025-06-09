/*!
# ch02 Rust基础概念

## 01 变量和可变性
1. [定义变量](./ch02_01_variable_mutability/fn.f01_01_variable.html)
2. [变量可变性](./ch02_01_variable_mutability/fn.f01_02_mutability.html)
3. [变量遮蔽](./ch02_01_variable_mutability/fn.f01_03_shadowing.html)

## 02 基本类型

1. [标量类型](./ch02_02_basic_type/fn.f02_01_scalar_type.html)
2. [组合类型](./ch02_02_basic_type/fn.f02_02_compound_type.html)

## [03 语句和表达式](./ch02_03_statement_expression/fn.statement_expression.html)

## [04 函数](./ch02_04_function/fn.function.html)

## 05 流程控制
1. [if else](./ch02_05_control_flow/fn.f05_01_if_else.html)
2. [for](./ch02_05_control_flow/fn.f05_02_for.html)
3. [while和loop](./ch02_05_control_flow/fn.f05_03_while_loop.html)
4. [continue和break](./ch02_05_control_flow/fn.f05_04_continue_break.html)
5. [label标识符](./ch02_05_control_flow/fn.f05_05_label.html)
*/

pub mod ch02_01_variable_mutability;
pub use ch02_01_variable_mutability::{f01_01_variable, f01_02_mutability, f01_03_shadowing};

pub mod ch02_02_basic_type;
pub use ch02_02_basic_type::{f02_01_scalar_type, f02_02_compound_type};

pub mod ch02_03_statement_expression;
pub mod ch02_04_function;

pub mod ch02_05_control_flow;
pub use ch02_05_control_flow::{
    f05_01_if_else, f05_02_for, f05_03_while_loop, f05_04_continue_break, f05_05_label,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch02_01() {
        assert_eq!(f01_01_variable(), ());
        assert_eq!(f01_02_mutability(), ());
        assert_eq!(f01_03_shadowing(), ());
    }

    #[test]
    fn ch02_02() {
        assert_eq!(f02_01_scalar_type(), ());
        assert_eq!(f02_02_compound_type(), ());
    }

    #[test]
    fn ch02_03() {
        assert_eq!(ch02_03_statement_expression::statement_expression(), ());
    }

    #[test]
    fn ch02_04() {
        assert_eq!(ch02_04_function::function(), ());
    }

    #[test]
    fn ch02_05() {
        assert_eq!(f05_01_if_else(), ());
        assert_eq!(f05_02_for(), ());
        assert_eq!(f05_03_while_loop(), ());
        assert_eq!(f05_04_continue_break(), ());
        assert_eq!(f05_05_label(), ());
    }
}
