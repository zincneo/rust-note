#![allow(dead_code)]
/**
# 语句和表达式
- Rust代码由语句和表达式组成
- 语句完成具体的操作但是没有返回值，以;结尾
  - 注意let关键字必须形成语句，没有返回值
- Rust中只要是返回值的都是表达式，表达式可以成为语句的一部分
  - 函数是表达式
  - {}是表达式
  - 表达式如果不返回值，就会隐式地返回一个空元组()
*/
pub fn f01_statement_expression() {
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
