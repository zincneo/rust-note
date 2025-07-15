#![allow(unused)]
#![allow(dead_code)]

/**
# 自定义类型使用运算符 + 为例
- Rust中对自定义类型要实现运算符的行为就要实现std::ops下对应的特征
- 例如对于自定义类型想要使用`+`运算符就要实现std::ops::Add
*/
pub fn f01_add() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Point(i32, i32);

    impl Add for Point {
        // 指定关联类型是当前类型
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    let p = Point(1, 2) + Point(2, 3);
}
