fn _ch09_01_unrecoverable() {
    /// ## 不可恢复的崩溃
    /// - panic宏可以主动让当前线程崩溃
    /// - rust不处理错误的时候会让程序崩溃
    /// - 数组越界访问会导致程序崩溃
    fn unrecoverable() {
        // 1. panic宏
        // panic!("crash and burn");
        // 2. 数组越界访问
        let v = vec![1, 2, 3];
        v[99];
    }
    unrecoverable();
}

fn _ch09_02_result() {
    /// ## 可恢复的错误
    /// - rust通过Result枚举来进行错误传播
    /// - 通过模式匹配的方式来处理错误
    /// - 传播错误的运算符?
    ///   - 只能在返回值类型为Result的函数体内使用
    ///   - 必须使用在一个返回值为Result的表达式之后
    ///   - 当一个表示式返回Result::Err的时候?运算符直接结束函数执行将Err传播给调用者
    ///   - 当一个表达式返回Ok的时候直接将Ok里面包裹的值解包返回
    fn result() {
        use std::fs::File;
        use std::io::{self, Read};
        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
        fn read_username_from_file() -> Result<String, io::Error> {
            // ? 运算符，错误的时候将错误传播给调度者，正确的时候直接解包返回继续执行函数体
            let mut username_file = File::open("hello.txt")?;
            let mut username = String::new();
            username_file.read_to_string(&mut username)?;
            Ok(username)
        }
        match read_username_from_file() {
            Ok(username) => {
                println!("{}", username)
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
    result();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch09_01() {
        assert_eq!(_ch09_01_unrecoverable(), ());
    }

    #[test]
    fn ch09_02() {
        assert_eq!(_ch09_02_result(), ());
    }
}
