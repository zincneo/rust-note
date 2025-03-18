/// ## 输出宏
/// - 最常用的几个输出宏
///   1. print! 将格式化文本输出到标准输出
///   2. println! 将格式化文本输出到标准输出并输出一个\n
///   3. format! 将格式化文本输出到String类型的字符串中
///   4. eprint! eprintln! 区别是输出到标准错误输出
fn _ch11_01_output_macro() {
    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");
    eprintln!("Error: Could not complete task");
}

/// ## 格式化占位符
/// - rust在格式化输出中提供了{}和{:?}作为占位符
/// - {}适用于std::fmt::Display特征的数据类型
/// - {:?}适用于std::fmt::Debug特征的数据类型
/// - {:#?}也适用于实现std::fmt::Debug特征的数据类型，更适合调试
/// - 实际上大多数标准库中提供的类型都实现了Debug特征
/// - 要在格式化输出的时候输出{}本身要使用{{}}
fn _ch11_02_placeholder() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
}

/// ## 实现Display和Debug特征的方式
/// - Debug特征特供了derive宏为自定义类型自动实现
/// - 由于孤儿规则，如果要为标准库中没有实现Display的类型实现Display则需要使用自建新类型包裹
/// - Display和Debug都需要实现一个fmt方法，该方法返回一个std::fmt::Result类型的值
/// - 可以使用write!宏返回std::fmt::Result类型的值
fn _ch11_03_impl_Dispaly_Debug() {
    use std::fmt::{Debug, Display};
    struct Position(i32, i32);
    impl Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.0, self.1)
        }
    }
    impl Debug for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Position {{ x: {}, y: {} }}", self.0, self.1)
        }
    }
    let pos = Position(0, 0);
    println!("{pos}");
    println!("{:?}", pos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch11_01() {
        assert_eq!(_ch11_01_output_macro(), ());
    }

    #[test]
    fn ch11_02() {
        assert_eq!(_ch11_02_placeholder(), ());
    }

    #[test]
    fn ch11_03() {
        assert_eq!(_ch11_03_impl_Dispaly_Debug(), ());
    }
}
