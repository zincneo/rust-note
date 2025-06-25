#![allow(dead_code)]
#![allow(unused)]

/**
# match
- Rust提供match关键字来匹配一个表达式和它对应的模式
- match匹配的格式`match expression { PATTERN => {}, ...}`
- match本身是一个表达式，每一个模式对应的代码块最后一个语句就是其返回值
- match必须是穷尽匹配，即必须匹配该类型的所有值
- `_`作为模式的时候是通配符，表示剩下所有的没有列出的可能性
- 在匹配结构体和枚举类型的时候可以通过对应的解构语法取到包裹字段的值
*/
pub fn f01_match() {
    // 1. 匹配枚举类型
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

    // 2. 匹配i32类型
    let num = 10;
    match num {
        1 => {
            println!("1");
        }
        _ => (),
    }

    // 3. 匹配枚举和结构体的时候配合对应的解构语法使用
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
*/
pub fn f02_if_let() {
    let num = Some(5);
    if let Some(3) = num {
        println!("3");
    } else {
        println!("other number");
    }
}

/**
# while let
- 和while循环功能类似但是由每次循环执行一次布尔值判断变为模式匹配
- `while let pattern = expression {}`
*/
pub fn f03_while_let() {
    #[derive(Debug)]
    enum CustomRange {
        A,
        B,
        C,
        D,
    }
    impl CustomRange {
        fn next(&self) -> Option<CustomRange> {
            use CustomRange::*;
            match self {
                A => Some(B),
                B => Some(C),
                C => Some(D),
                D => None,
            }
        }
    }
    let mut range = Some(CustomRange::A);
    while let Some(ref tmp) = range {
        println!("{:?}", tmp);
        range = tmp.next();
    }
}

/**
# matches宏
- 用来将一个表达式和一个模式进行匹配
- 匹配结果返回true和false
- 同样很适合那些没有实现`==`类型使用
- 下面的例子使用了迭代器和闭包，作用是根据filter传入的闭包来筛选想要的值，这个闭包需要返回一个布尔值
*/
pub fn f04_matches() {
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let _v: Vec<MyEnum> = v
        .into_iter()
        .filter(|ele| matches!(ele, MyEnum::Foo))
        .collect();
    let bar = MyEnum::Bar;
    let res = matches!(bar, MyEnum::Foo);
}
