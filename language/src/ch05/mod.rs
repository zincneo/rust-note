/*!
# ch05 Rust方法

## [01 方法语法](./fn.f01_method.html)

## [02 关联函数](./fn.f02_associative_function.html)
*/

/**
# 方法语法
- Rust中可以为结构体和枚举实现方法(method)
    - 所谓方法和函数的差距其实就是面向对象程序中的封装概念的体现
    - 函数用来表示不与某一类的具体实例关联
    - 方法用来表示与某一类的具体实例关联
- 使用关键字impl来定义实现方法的代码块`impl StructName/EnumName {}`
- Self关键字在impl块中表示结构体和枚举类型本身
- 方法的第一个参数为self,&self,&mut self三种
    1. self 方法直接拿走调用该方法的实例的所有权
    2. &self 方法获取调用实例的不可变借用
    3. &mut self 方法获取调用实例的可变借用
- Rust中允许方法和结构体中字段的名称相同
- 一个结构体或者枚举可以有多个impl块
- Rust中没有`->`运算符，再后续章节中解释解引用行为中详细描述
```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
```
*/
#[allow(dead_code)]
pub fn f01_method() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

/**
# 关联函数
- 在impl块中的第一个参数不是self,&self,&mut self称为关联函数
- 关联函数不能通过实例.函数名进行调用，需要使用类型名::函数名进行调用
```rust
impl Rectangle {
    // new是一个关联函数
    fn new(w: u32, h: u32) -> Self {
        Rectangle { width: w, height: h }
    }
}
```
*/
#[allow(dead_code)]
pub fn f02_associative_function() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn new(w: u32, h: u32) -> Self {
            Rectangle {
                width: w,
                height: h,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch05_01() {
        assert_eq!(f01_method(), ());
    }

    #[test]
    fn ch05_02() {
        assert_eq!(f02_associative_function(), ());
    }
}
