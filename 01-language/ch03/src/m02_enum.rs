#![allow(dead_code)]

/**
# 枚举语法
- Rust中使用enum关键字来定义枚举
- 枚举名和枚举值遵循大驼峰规范
- 通过`枚举名::枚举值`来获取到枚举值
- Rust中的枚举很强大，枚举值可以包裹值，可以把枚举值当作结构体定义
    - 相当于同时具备了c中枚举和联合类型的能力
- 获取枚举值的包裹的值需要使用到模式匹配，在模式匹配章节中详细说明
- Rust枚举的常见用途
    - 函数传递参数时希望传递不同类型时做类型同一化
    - 标准库提供的Option枚举起到c++中nullptr的作用
    - 标准库提供的Result枚举起到错误传播和处理的作用
- Rust也支持传统的枚举值映射到整型值的枚举
    - 只要所有的枚举值不包裹其他值就是传统的枚举
    - 映射到整型数字从0开始递增或使用=绑定到指定的整型值
    - 枚举值可以使用as关键字强制类型转换为整数
    - 整数无法使用as关键字转换为枚举值
*/
pub fn f01_enum() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::Write(String::from("message"));
    let m4 = Message::ChangeColor(255, 255, 0);
    println!("{:?}\n{:?}\n{:?}\n{:?}", m1, m2, m3, m4);

    enum Number {
        Zero = 0,
        One = 1,
    }

    let num = Number::Zero as i32;
    println!("{}", num);
}

/**
# Option枚举
- Rust标准库提供的一个枚举，该枚举接收一个泛型参数
- 该枚举有两个枚举值，Some(T)和None
- None的作用c++中的空指针nullptr是类似的，这种设计可以避免空指针调用方法的null异常
- 本质上是对普通值的封装，编译器确保了在使用值的时候永远不会用到空值
```rust
#[derive]
enum Option<T> {
    Some(T),
    None
}
```
*/
pub fn f02_option() {
    let mut some_number = Some(5);
    println!("{:?}", some_number);
    some_number = None;
    println!("{:?}", some_number);
}

/**
# Result枚举
- Rust标准库提供的一个枚举，该枚举接收两个泛型参数
- 该枚举有两个枚举值，Ok(T)和Err(E)
- Result枚举用于错误处理，详细见错误处理章节
*/
pub fn f03_result() {
    let _success: Result<i32, ()> = Ok(0);
    let _error: Result<i32, ()> = Err(());
}
