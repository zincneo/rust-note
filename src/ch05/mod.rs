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

fn _ch05_02_match() {
    /// ## 模式匹配
    /// - rust中提供的match关键字可以用来实现强大的流程控制功能
    /// - match的用法就是匹配类型到值的映射
    /// - 注意match必须要穷尽匹配，即必须匹配该类型的所有可能的值
    /// - _可以用来表示该类型剩余未列出的所有值
    /// - 可以使用变量名起到和_一样的效果，不同是在对应的模式内可以使用变量名表示匹配到的值
    /// - 匹配的时候变量名可以通过@绑定到具体的值，或者一个range
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn match_sample() {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        let coin = Coin::Dime;
        let value = match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };
        println!("{value}");
        let num = Some(100);
        match num {
            Some(_) => (),
            None => (),
        }
        match num {
            Some(i) => {
                println!("{i}");
            }
            None => (),
        }
        match num {
            Some(i @ 1_i32..=100_i32) => {
                println!("{i}");
            }
            Some(i @ 101_i32) => {
                println!("best");
            }
            Some(_) => (),
            None => (),
        }
    }
    match_sample();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch05_01() {
        assert_eq!(_ch05_01_enum(), ());
    }

    #[test]
    fn ch05_02() {
        assert_eq!(_ch05_02_match(), ());
    }
}
