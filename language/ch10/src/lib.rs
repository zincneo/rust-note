#![allow(dead_code)]
#![allow(unused)]
/*!
# Rust格式化输出
1. 常用的宏
2. 宏中的占位符
3. 占位符中的参数
4. 调试常用的几个宏
*/

/**
# 常用的宏
- print! 将格式化文本输出到标准输出，不带换行符
- println! 同上，但是在行的末尾添加换行符
- eprint! 将格式化文本输出到标准错误输出，不带换行符
- eprintln! 同上，但是在行的末尾添加换行符
- format! 返回String类型的值
- 以上的宏都接收不定长的参数，第一个参数是一个格式化字符串str字面值
*/
pub fn f01_macro() {
    print!("hello");
    println!("hello");
    eprint!("hello");
    eprintln!("hello");
    let mut hello = format!("hello");
}

/**
# 占位符
- Rust选择使用`{}`作为格式化字符串中的占位符
- `{}`会将对应位置的参数转换为字符串填入格式化字符串，需要实现特征Display才可以被转换
- `{:?}`/`{:#?}`也是占位符，对应的参数必须实现特征Debug
*/
pub fn f02_placeholder() {
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    println!("{} {:?} {:#?}", i, s, v);
}

/**
# 占位符中的参数
1. 位置参数
    - `{0}`指定使用的参数索引，下标从0开始
2. 具名参数
    - 格式化的字符串之后的参数可以使用`argument_name = expression`来给参数命名
    - 必须放在非具名参数之后
3. 格式化参数
    1.填充`{:[填充字符][对齐方式][宽度]}`
        1. 对齐方式: `<`左对齐(右边填充) `^`居中对齐 `>`右对齐(左边填充)
        2. 宽度: 使用索引和具名参数后面需要加上$符号，如要第一个参数作为宽度要写作`0$`
        3. 字符串默认填充字符是空字符，数字默认填充字符是0
    2. 精度`{:.保留的小数位}`
        - 保留的小数位同样可以使用索引$和具名参数$指定
    3. 进制`{:#进制字符}`
        - `#b` `#o` `#x` `#X`
        - `x` 特殊，不需要加#，表示不带前缀的16进制
    4. 指数表示`{:e}`
        - `#E` `#e` `E` `e`
    5. 指针地址`{:p}`
    6. 转义字符
        - 使用`\`来转义字符
        - 使用`{{`来转义`{`
        - 使用`}}`来转义`}`
    7. 在格式化字符串捕获当前环境中变量的值`{variable_name}`
*/
pub fn f03_arguments() {
    // 1. 位置参数
    {
        println!("{1}{0}", 1, 2); // =>"21"
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
        println!("{1}{}{0}{}", 1, 2); // => 2112
    }

    // 2. 具名参数
    {
        println!("{argument}", argument = "test"); // => "test"
        println!("{name} {}", 1, name = 2); // => "2 1"
        // println!("{abc} {1}", abc = "def", 2); // 报错
        let x = 10;
        println!("{name}", name = x);
    }

    // 3. 格式化参数
    {
        // 1. 填充
        {
            // 为"x"后面填充空格，补齐宽度5
            println!("Hello {:5}!", "x");
            // 为"x"后填充@字符，补齐宽度5
            println!("Hello {:@>5}!", "x");
            // 使用参数5来指定宽度
            println!("Hello {:1$}!", "x", 5);
            // 使用x作为占位符输出内容，同时使用5作为宽度
            println!("Hello {1:0$}!", 5, "x");
            // 使用有名称的参数作为宽度
            println!("Hello {:width$}!", "x", width = 5);
        }

        // 2. 精度
        {
            let v = 3.1415926;
            // 保留小数点后两位 => 3.14
            println!("{:.2}", v);
            // 带符号保留小数点后两位 => +3.14
            println!("{:+.2}", v);
            // 不带小数 => 3
            println!("{:.0}", v);
            // 通过参数来设定精度 => 3.1416，相当于{:.4}
            println!("{:.1$}", v, 4);
            // 通过具名参数来设定精度
            println!("{:.len$}", v, len = 4);
        }

        // 3. 进制
        {
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

            // 将ASCII码字符输出为16进制数值
            println!("{:#X}", b'c');
        }

        // 4. 指数
        {
            println!("{:#e}", 1000000000); // => 1e9
            println!("{:#E}", 1000000000); // => 1E9
            println!("{:e}", 1000000000); // => 1e9
            println!("{:E}", 1000000000); // => 1E9
        }

        // 5. 指针地址
        {
            let v = vec![1, 2, 3];
            println!("{:p}", v.as_ptr()) // => 0x600002324050
        }

        // 6. 转义字符
        {
            println!("\\");
            println!("{{}}");
        }

        // 7. 捕获环境中的变量
        {
            let p = 5;
            println!("Hello, {}!", p); // 自动递增参数
            println!("Hello, {0}!", p); // 索引
            println!("Hello, {person}!", person = p); // 具名参数
            println!("Hello, {p}!"); // 直接捕获当前环境中的变量p
        }
    }
}

/**
# 调试常用的几个宏
- `line!()`返回u32值，表示当前代码的行号
- `file!()`返回&str，表示当前的文件的完整路径
- `module_path!()`返回&str，表示当前代码所在的模块路径
*/
pub fn f04_macro() {
    let line = line!();
    let file = file!();
    let path = module_path!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_format() {
        assert_eq!(f01_macro(), ());
        assert_eq!(f02_placeholder(), ());
        assert_eq!(f03_arguments(), ());
        assert_eq!(f04_macro(), ());
    }
}
