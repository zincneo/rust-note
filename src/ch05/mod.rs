fn _ch05_01_enum() {
    /// ## 枚举
    /// - rust使用关键字enum定义枚举
    /// - 当所有枚举可能性不赋值的时候类似c的枚举，每个枚举可能性按照定义的顺序映射到一个整型值，从0开始
    /// - rust的枚举特殊在可以包含值，用法就和将枚举的可能性当作结构体的类型名一样
    /// - Option枚举是标准库提供用来解决空指针问题的一个枚举
    fn defining_enum() {
        #[allow(dead_code)]
        enum IpAddrKind {
            V4,
            V6,
        }
        let v4 = IpAddrKind::V4 as i32;
        println!("{v4}");
        #[allow(dead_code)]
        enum Message {
            Quit,                       // struct Quit;
            Move { x: i32, y: i32 },    // struct Move {x: i32, y: i32}
            Write(String),              // struct Write(String)
            ChangeColor(i32, i32, i32), // struct ChangeColor(i32, i32, i32)
        }
        // Option<T>枚举
        // enum Option<T> {
        //     Some(T),
        //     None
        // }
        let mut num = Some(6);
        println!("{:#?}", num);
        // rust没有nullptr，表示一个值现在不存在使用Option::None
        num = None;
        println!("{:#?}", num);
    }
    defining_enum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch05_01() {
        assert_eq!(_ch05_01_enum(), ());
    }
}
