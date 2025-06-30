#![allow(dead_code)]
#![allow(unused)]

use std::io::Read;
/**
# 不可恢复的错误
- 当代码执行必然导致当前程序无法正常运行或者会对系统造成破坏的时候使用`panic`宏直接让程序或者线程崩溃
    - 举例来说，系统启动阶段读取必要的文件失败，系统无法启动，这属于不可恢复的错误，直接触发panic
- 被动触发
    - 如数组的越界访问，Rust认为会对系统内存数据造成破坏，主动让程序崩溃
- 主动触发
    - 代码中调度panic宏
- 运行项目时配置环境变量使得崩溃的时候打开函数调用栈
    `RUST_BACKTRACE=1 cargo run`
*/
pub fn f01_panic() {
    let v = [1, 2, 3];
    // v[5]; // 越界访问造成运行时程序崩溃
    // panic!("crash and burn"); // 主动让程序执行到该位置崩溃
}

/**
# 可恢复的错误
- 标准库提供了枚举类型Result
- Result::Ok值用来包裹程序运行正确的情况下的值，Result::Err值用来包裹程序运行失败情况下的信息
- 通过模式匹配match来处理成功和失败的情况
- Result枚举本身包含在标注库std::result中
    - 该枚举在prelude中包含，因此可以直接使用
- 该类型专门用来处理在程序执行过程中认为是可以被处理的错误
- Result类型并非一定要处理，如果不想对错误进行处理可以调用unwrap方法，方法会返回Ok中包裹的值，对于Err则直接按照不可恢复错误处理，触发panic
*/
pub fn f02_result() {
    use std::fs::File;

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // 直接返回不继续向后执行也是一种错误处理
            return;
        }
    };
}

/**
# 错误传播
- 当一个函数的返回值为Result枚举或者Option枚举的函数体内，可以使用`?`向上级传播错误
- 对于Ok和Some枚举值，`?`会解除值的包裹，对于Err和None，将会直接返回Err和None
- 可以认为就是match的语法糖
- 因此`?`的作用就是当Result为Ok值的时候直接解构出包裹的值，当Result为Err的时候结束函数，向上传播错误
- 对于Option枚举使用?则Option为Some值的时候解构出包裹的值，当Option为None的时候结束函数，返回None
- Rust的主函数还支持返回值为`Result<(), Box<dyn std::error::Error>>`类型
*/
pub fn f03_question_mark() {
    use std::fs::File;
    use std::io;
    fn read_username_from_file_1() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    // 不使用?语法糖
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(err) => return Err(io::Error::new(io::ErrorKind::NotFound, err)),
        }
        Ok(s)
    }
    // 链式调用
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    // Option也可以使用?运算符
    fn option_demo() -> Option<i32> {
        let warp: Option<i32> = None;
        let content = warp?;
        Some(content + 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_panic() {
        assert_eq!(f01_panic(), ());
    }

    #[test]
    fn test02_result() {
        assert_eq!(f02_result(), ());
    }

    #[test]
    fn test03_question_mark() {
        assert_eq!(f03_question_mark(), ());
    }
}
