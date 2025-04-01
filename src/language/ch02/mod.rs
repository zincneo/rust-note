/*!
# ch02 Rust基础概念

## 01 变量和可变性
1. [定义变量](./ch02_01_variable_mutability/fn.f01_01_variable.html)
2. [变量可变性](./ch02_01_variable_mutability/fn.f01_02_mutability.html)
3. [变量遮蔽](./ch02_01_variable_mutability/fn.f01_03_shadowing.html)

## 02 基本类型

1. [标量类型](./ch02_02_basic_type/fn.f02_01_scalar_type.html)
2. [组合类型](./ch02_02_basic_type/fn.f02_02_compound_type.html)

## [03 语句和表达式](./fn.f03_statement_expression.html)

## [04 函数](./fn.f04_function.html)

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

/**
# 语句和表达式
- Rust代码由语句和表达式组成
- 语句完成具体的操作但是没有返回值，以;结尾
  - 注意let关键字必须形成语句，没有返回值
- Rust中只要是返回值的都是表达式，表达式可以成为语句的一部分
  - 函数是表达式
  - {}是表达式
  - 表达式如果不返回值，就会隐式地返回一个空元组()

```rust
let _a = 10; // 变量定义语句
let x = 1;
// 有返回值的就是表达式
let _z = if x % 2 == 1 { "odd" } else { "even" };
```
*/
pub fn f03_statement_expression() {
    // 1. 语句
    {
        let _a = 10; // 变量定义语句

        // let a = (let a = 8); // let必须是语句，语句没有返回值，不能用作变量绑定
    }
    // 2. 表达式
    {
        let x = 1;
        // 有返回值的就是表达式
        let _z = if x % 2 == 1 { "odd" } else { "even" };
    }
}

/**
# 函数

## 函数要点
1. Rust使用fn关键字来定义函数
2. 函数名命名规则与变量相同
3. 函数位置可以随意放置，Rust不在乎在哪里定义，有定义即可
4. 函数定义形式`fn add(a: i32, b: i32) -> i32 {}`

## 参数列表
- Rust是强类型语言，必须为每一个参数标明类型

## 函数返回值
- 在参数列表后使用-> return_type注明返回值类型
- 当不标准返回值类型的时候等价与-> () 返回空元组
- 在{}函数体中可以使用return关键字来返回值也可以将最后一个表达式作为返回值，如果最后是一个语句且不是return语句则返回空元组

## 发散函数
- 返回值类型为!表示这个函数是一个发散函数
- 发散函数的含义是无法回到调用点继续执行代码
- 通常用于会导致死循环或者程序崩溃的函数
*/
pub fn f04_function() {
    #[allow(unused)]
    {
        fn dead_end() -> ! {
            panic!("你已经到了穷途末路，崩溃吧！");
        }

        fn forever() -> ! {
            loop {
                //...
            }
        }
    }
}

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
        assert_eq!(f03_statement_expression(), ());
    }

    #[test]
    fn ch02_04() {
        assert_eq!(f04_function(), ());
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
