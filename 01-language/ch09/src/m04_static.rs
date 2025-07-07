#![allow(dead_code)]
#![allow(unused)]
/**
# 静态生命周期

- `'static`生命周期标识符可以让变量活得和程序一样久
- 字符串字面量，提到过它是被硬编码进 Rust 的二进制文件中，因此这些字符串变量全部具有 'static 的生命周期
- 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static，有时候它会给你奇迹
*/
pub fn f01_static() {
    let s: &'static str = "test";
    println!("{s}");
}

/**
# 一个相对复杂的例子
- 在一个函数中同时包含泛型，生命周期标识符，特征约束
*/
pub fn f02_case() {
    use std::fmt::Display;
    fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() { x } else { y }
    }
}
