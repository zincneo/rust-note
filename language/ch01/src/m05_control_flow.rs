#![allow(dead_code)]

/**
# if else控制表达式
- Rust中使用if/else关键字来控制代码执行流
- if 和 else if 之后跟上一个会返回布尔类型的表达式，该表达式返回值为true的情况下会执行对于的代码块
- Rust中代码块`{}`也是表达式，因此if else 表达式会返回代码块`{}`的返回值
- Rust处理分支情况更加复杂的情况提供了match模式匹配来取代if else，其能力要强大很多，详细见模式匹配章节
*/
pub fn f01_if_else() {
    let mut _num = 50;
    let _n = if {
        _num /= 10;
        true
    } {
        100
    } else {
        90
    };
}
