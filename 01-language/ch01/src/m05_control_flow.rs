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

/**
# match和if let
- Rust中提供了强大的模式匹配，其中match关键字提供了匹配一个表达式所有值的能力`match expression { value => {}, ... }`
- match的最基础用法可以替代掉其他语言中的switch case，当然这比switch case的能力强大太多，因此Rust也不提供switch case关键字
- match 必须穷尽匹配
    - 使用`_`可以匹配剩余的所有情况
- 如果只想匹配一种值的情况可以使用if let `if let value = expression`
- match有更多详细用法见模式匹配章节
*/
pub fn f02_match() {
    let num = 10;
    // 通过match匹配一个返回i32的表达式
    match num + 10 {
        20 => (), // 分支匹配到i32类型的值
        _ => (),  // 必须穷尽匹配
    }
    // if let 可以只匹配一种值
    if let 20 = num + 10 {}
}
