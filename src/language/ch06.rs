/*!
# ch06 Rust模式匹配

## 01 match和if let
1. [match](./fn.f01_01_match.html)
2. [if let](./fn.f01_02_if_let.html)
3. [matches!](./fn.f01_03_matches.html)
4. [解构Option枚举](./fn.f01_04_option.html)
##
*/

/**
# match
- Rust提供match关键字用来将一个值和一系列值进行匹配
```
// 通用的形式
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```
- match本身是一个表达式，每一个模式对应的代码块最后一个语句就是其返回值
- match宏必须是穷尽匹配的，也就是要匹配该类型的所有值
- `_`作为模式的时候是通配符，表示剩下所有的没有列出的可能性
- 以下看几个匹配的例子
```rust
    // 匹配枚举值
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);
```
```rust
    // 匹配整数值
    let num = 10;
    match num {
        1 => {
            println!("1");
        }
        _ => (),
    }
```
- 在匹配枚举包裹值的情况下，可以通过模式取出包裹的值
    - 枚举值定义`EnumValue(type1, type2, type3)`则对应的模式为`EnumValue(name1, name2, name3)`
```
    enum Message {
        Quit,
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let message = Message::ChangeColor(255, 0, 122);
    match message {
        Message::Quit => (),
        Message::Write(s) => {
            println!("{s}");
        }
        Message::ChangeColor(_r, _g, _b) => (),
    }
```
*/
#[allow(dead_code)]
pub fn f01_01_match() {
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);
    let num = 10;
    match num {
        1 => {
            println!("1");
        }
        _ => (),
    }
    enum Message {
        Quit,
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let message = Message::ChangeColor(255, 0, 122);
    match message {
        Message::Quit => (),
        Message::Write(s) => {
            println!("{s}");
        }
        Message::ChangeColor(_r, _g, _b) => (),
    }
}

/**
# if let
- if let用于只想匹配一种模式的情况
- `if let 模式 = 值 {}`
- 很适合适用于不能使用`==`运算符判等的类型替代if
- 支持else块
```rust
    let num = Some(5);
    if let Some(3) = num {
        println!("3");
    } else {
        println!("other number");
    }
```
*/
pub fn f01_02_if_let() {
    let num = Some(5);
    if let Some(3) = num {
        println!("3");
    } else {
        println!("other number");
    }
}

/**
# matches宏
- 用来将一个表达式和一个模式进行匹配
- 匹配结果返回true和false
- 同样很适合那些没有实现`==`类型使用
- 下面的例子使用了迭代器和闭包，作用是根据filter传入的闭包来筛选想要的值，这个闭包需要返回一个布尔值
```rust
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let _v: Vec<MyEnum> = v
        .into_iter()
        .filter(|ele| matches!(ele, MyEnum::Foo))
        .collect();
```
*/
pub fn f01_03_matches() {
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let _v: Vec<MyEnum> = v
        .into_iter()
        .filter(|ele| matches!(ele, MyEnum::Foo))
        .collect();
}

/**
# 解构Option枚举
- 由于Option过于常用，因此Rust在prelude中进行了导出，因此可以直接使用Some和None，不需要使用Option::Some,Option::None
- Rust通过模式匹配的方式来解决其他语言中null异常的问题
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 不存在值的情况就不会触发处理逻辑
        Some(i) => Some(i + 1), // 能够匹配到实际的对象再调用方法
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
*/
pub fn f01_04_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,           // 不存在值的情况就不会触发处理逻辑
            Some(i) => Some(i + 1), // 能够匹配到实际的对象再调用方法
        }
    }
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch06_01() {
        assert_eq!(f01_01_match(), ());
        assert_eq!(f01_02_if_let(), ());
        assert_eq!(f01_03_matches(), ());
        assert_eq!(f01_04_option(), ());
    }
}
