/*!
# ch05 Rust方法

## [01 方法定义](./fn.f01_method.html)

## [02 关联函数](./fn.f02_associative_function.html)
*/

/**
# 方法
- Rust中可以为结构体和枚举实现方法
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch05_01() {
        assert_eq!(f01_method(), ());
    }
}
