#![allow(dead_code)]
#![allow(unused)]

/**
# 读写特征
- Read和Write特征统一接口并为其他类型实现
    1. `fs`模块下的File类型
    2. `net`模块下的TcpStream类型
    3. `Vec<T>`
    4. `io`模块下的Stdin、Stdout等结构体
- Read特征最主要的方法是read，读取到传入的&mut [u8]
- Write特征最主要的方法是write，将传入的&[u8]写入
*/
pub fn f01_read_write_trait() {
    // 1. 为自定义类型实现Read
    {
        use std::io::Read;
        struct Custom(Vec<char>);
        impl Read for Custom {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                if self.0.len() > buf.len() {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "other".to_string(),
                    ))
                } else {
                    for (idx, c) in self.0.iter().enumerate() {
                        buf[idx] = (((*c) as u64) % 128) as u8;
                    }
                    self.0.clear();
                    Ok(self.0.len())
                }
            }
        }

        let mut custom = Custom("abc".chars().into_iter().collect());

        let mut buffer = [0; 10];

        custom.read(&mut buffer);

        println!("{:?}", buffer);
    }
    // 2. 为自定义类型实现Write
    {
        use std::fmt::Display;
        use std::io::Write;
        struct Custom(Vec<char>);
        impl Display for Custom {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.iter().copied().collect::<String>())
            }
        }
        impl Write for Custom {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                for c in buf {
                    self.0.push(*c as char)
                }
                Ok(buf.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut custom = Custom(Vec::new());
        let mut buffer = [97, 98, 99];
        custom.write(&mut buffer);
        println!("{}", custom);
    }
}

/**
# Seek和BufRead
- Seek特征提供seek方法，作用是跳转到指定的位置
- BufRead使用内部缓冲区来提供许多其他读取方式
    - Read提供的接口是基于u8(一个byte)效率低下
    - BufReader和BufWriter结构体内部使用缓冲区来减少调用次数
*/
pub fn f02_seek_bufread() {
    // 1. Seek
    {
        use std::fs::File;
        use std::io::{Read, Seek, SeekFrom};
        let mut f = File::open("foo.txt").unwrap();
        let mut buffer = [0; 10];

        // 跳到文件的最后 10 个字节
        f.seek(SeekFrom::End(-10)).unwrap();

        // 最多读取 10 个字节
        let n = f.read(&mut buffer).unwrap();

        println!("The bytes: {:?}", &buffer[..n]);
    }
    // 2. BufReader/BufWrite
    {
        use std::fs::File;
        use std::io;
        use std::io::BufReader;
        use std::io::prelude::*;
        let f = File::open("foo.txt").unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();

        // 将一行读入缓冲区
        reader.read_line(&mut buffer).unwrap();

        println!("{buffer}");
    }
}

/**
# io模块下的Result
- io模块下的定义`pub type Result<T> = result::Result<T, Error>;`
- 这里泛型参数中的Error是io模块下定义的Erro结构体
- 该枚举主要用于错误传播的时候简化错误类型
*/
pub fn f03_io_result() {
    use std::io;

    fn read_input() -> io::Result<()> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        println!("You typed: {}", input.trim());

        Ok(())
    }
}
