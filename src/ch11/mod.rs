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
/// - 实际上大多数标准库中提供的类型都实现了Debug特征
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
}
