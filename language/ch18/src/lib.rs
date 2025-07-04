#![allow(dead_code)]
#![allow(unused)]

mod m01_trait_bound;
mod m02_associate_type;

/**
# 父特征
- 与限定泛型参数的语法类似，在定义的特征的时候也可以限定特征
- `trait A: B` B就是A的父特征，那么在为类型实现A特征时必须要求类型实现了B特征
*/
pub fn f03_super_trait() {
    // 一个类型要实现OutlinePrint特征就必须实现Display特征
    trait OutlinePrint: std::fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
}

/**
# newtype惯例用法
- newtype含义指通过元组结构体包裹一种原有的类型
- 例如: `struct Speed(i32)`
- 这样的做法既可以包含原有类型的所有方法又可以为新类型实现方法，此外还可以为newtype实现特征
- Rust中实现特征`impl Trait for T0`必须遵循孤儿规则，满足以下两条中的一条
    1. Trait是当前的crate中定义的
    2. 类型是本地定义的
- 简单来说孤儿规则要求的就是不能在当前的crate中为crate之外的类型实现crate之外的特征
- 因此newtype包裹可以让crate外的类型变成本地类型，实现特征就可以随意了
*/
pub fn f04_newtype() {
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_trait_bound() {
        assert_eq!(m01_trait_bound::f01_trait_bound(), ());
        assert_eq!(m01_trait_bound::f02_multiple_bound(), ());
        assert_eq!(m01_trait_bound::f03_where(), ());
    }

    #[test]
    fn test02_associate_type() {
        assert_eq!(m02_associate_type::f01_associate_type(), ());
        assert_eq!(m02_associate_type::f02_case(), ());
    }

    #[test]
    fn test03_super_trait() {
        assert_eq!(f03_super_trait(), ());
    }

    #[test]
    fn test04_newtype() {
        assert_eq!(f04_newtype(), ());
    }
}
