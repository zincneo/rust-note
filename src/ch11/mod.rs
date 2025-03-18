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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch11_01() {
        assert_eq!(_ch11_01_output_macro(), ());
    }
}
