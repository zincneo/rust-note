/// ## 输出宏
/// - 最常用的几个输出宏
///   1. print! 将格式化文本输出到标准输出
///   2. println! 将格式化文本输出到标准输出并输出一个\n
///   3. format! 将格式化文本输出到String类型的字符串中
///   4. eprint! eprintln! 区别是输出到标准错误输出
fn _ch11_01_output_macro() {
    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");
    eprintln!("Error: Could not complete task");
}

/// ## 格式化占位符
/// - rust在格式化输出中提供了{}和{:?}作为占位符
/// - {}适用于std::fmt::Display特征的数据类型
/// - {:?}适用于std::fmt::Debug特征的数据类型
/// - {:#?}也适用于实现std::fmt::Debug特征的数据类型，更适合调试
/// - 实际上大多数标准库中提供的类型都实现了Debug特征
/// - 要在格式化输出的时候输出{}本身要使用{{}}
fn _ch11_02_placeholder() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
}

/// ## 实现Display和Debug特征的方式
/// - Debug特征特供了derive宏为自定义类型自动实现
/// - 由于孤儿规则，如果要为标准库中没有实现Display的类型实现Display则需要使用自建新类型包裹
/// - Display和Debug都需要实现一个fmt方法，该方法返回一个std::fmt::Result类型的值
/// - 可以使用write!宏返回std::fmt::Result类型的值
fn _ch11_03_impl_Dispaly_Debug() {
    use std::fmt::{Debug, Display};
    struct Position(i32, i32);
    impl Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.0, self.1)
        }
    }
    impl Debug for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Position {{ x: {}, y: {} }}", self.0, self.1)
        }
    }
    let pos = Position(0, 0);
    println!("{pos}");
    println!("{:?}", pos);
}

/// ## 格式化占位符参数
/// - 位置参数，指定某一个位置的参数填充某个占位符如{0} {1}，参数下标从0开始
/// - 具名参数，带名称的参数必须放在不带名称参数的后面，否则会报错
/// - 格式化参数 :之后
///   1. 宽度
///   2. 精度
///   3. 进制
///   4. 指数
///   5. 指针地址
///   6. 转义
fn _ch11_04_placeholder_argument() {
    // 位置参数
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
    println!("{1}{}{0}{}", 1, 2); // => 2112

    // 具名参数
    let a = 100;
    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
    println!("{a}"); // => 100 参数列表不给出会通过闭包找到同名变量

    // 格式化参数
    {
        // 1. 宽度 {:[填充字符][对齐方式][宽度]}
        {
            // 1. 默认使用空格补齐，传入数字作为补齐的长度
            println!("Hello {:5}!", "x"); // =>  "Hello x    !"

            // 2. 使用位置参数$来指定补齐多长
            println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

            // 3. 使用位置参数:位置参数$同时指定占位符输出的内容和长度
            println!("Hello {0:1$}!", "x", 5); // =>  "Hello x    !"

            // 4. 使用具名参数$来指定占位符输出的长度
            println!("Hello {:width$}!", "x", width = 5); // =>  "Hello x    !"

            // 5. 字符串和数字都默认用空格填充，但是字符串左对齐，数字右对齐
            println!("Hello {:5}!", 5); // => "Hello     5!"

            // 6. 指定对齐方式>右对齐 <左对齐 ^居中对齐
            println!("Hello {:^5}!", "x"); // => "Hello   x  !"

            // 7. 指定填充字符
            println!("Hello {:@^5}!", "x"); // => "Hello @@x@@!"
        }

        // 2. 精度
        {
            // 通过.数字来指定小数点后保留几位
            let v = 3.1415926;
            // 保留小数点后两位 => 3.14
            println!("{:.2}", v);
            // 带符号保留小数点后两位 => +3.14
            println!("{:+.2}", v);
            // 不带小数 => 3
            println!("{:.0}", v);
            // 通过参数来设定精度 => 3.1416，相当于{:.4}
            println!("{:.1$}", v, 4);
        }

        // 3. 进制
        {
            // #b二进制 #o八进制 #x小写十六进制 #X大写十六进制 x不带前缀的十六进制
            // 二进制 => 0b11011!
            println!("{:#b}!", 27);
            // 八进制 => 0o33!
            println!("{:#o}!", 27);
            // 十进制 => 27!
            println!("{}!", 27);
            // 小写十六进制 => 0x1b!
            println!("{:#x}!", 27);
            // 大写十六进制 => 0x1B!
            println!("{:#X}!", 27);
            // 不带前缀的十六进制 => 1b!
            println!("{:x}!", 27);
            // 使用0填充二进制，宽度为10 => 0b00011011!
            println!("{:#010b}!", 27);
        }

        // 4. 指数
        {
            println!("{:2e}", 1000000000); // => 1e9
            println!("{:2E}", 1000000000); // => 1E9
        }

        // 5. 指针地址
        {
            let v = vec![1, 2, 3];
            println!("{:p}", v.as_ptr()); // 十六进制显示32bit的数值
        }

        // 6. 转义
        {
            // 和其他语言一样使用\进行转义
            // {}被占位符使用，因此要使用{{转义{，}}转义}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch11_01() {
        assert_eq!(_ch11_01_output_macro(), ());
    }

    #[test]
    fn ch11_02() {
        assert_eq!(_ch11_02_placeholder(), ());
    }

    #[test]
    fn ch11_03() {
        assert_eq!(_ch11_03_impl_Dispaly_Debug(), ());
    }

    #[test]
    fn ch11_04() {
        assert_eq!(_ch11_04_placeholder_argument(), ());
    }
}
