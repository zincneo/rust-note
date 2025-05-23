/*!
# Rust读写文件

1. [打开/创建文件](./fn.f01_open_create.html)
2. [读取文件](./fn.f02_read.html)
3. [写入文件](./fn.f03_write.html)
*/

use std::io::{Read, Write};

/**
# 打开/创建文件
1. 判断文件是否存在
    - 标准库下的path模块中提供了路径类型Path
    - 该类型通过new方法将字符串转换为Path的引用类型
    - Path上有一系列方法用来判断路径是否存在以及是文件类型
    ```rust
    const PATH: &str = "file.txt";
    let path = std::path::Path::new(PATH);
    if path.exists() {
        println!("指定路径存在");
    } else {
        println!("指定路径不存在");
    }
    if path.is_file() {
        println!("指定路径是文件");
    } else if path.is_dir() {
        println!("指定文件是文件夹");
    } else if path.is_symlink() {
        println!("指定文件是连接文件");
    }
    ```
2. 打开/创建文件
    - 标注库下的fs模块提供了文件类型File
    - File::open / File::create
    - open方法会以可读模式打开文件
    - create方法会创建文件，若文件存在则清空文件内容，然后以可写方式打开
    - create_new方法会创建文件，若文件存在则返回Err
    ```rust
    if path.exists() {
        match std::fs::File::open(&PATH) {
            Ok(_) => (),
            Err(_) => {}
        }
    } else {
        match std::fs::File::create(&PATH) {
            Ok(_) => (),
            Err(_) => {}
        }
    }
    ```
3. 打开文件的模式
    - 标准库下的fs模块提供访问控制类型OpenOptions
    - 通过new方法创建实例
    - 提供read/write/append方法设置模式
    - 提供open方法打开指定路径的文件
    ```rust
    let _ = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path);
    ```
*/
pub fn f01_open_create() {
    const PATH: &str = "file.txt";
    let path = std::path::Path::new(PATH);
    if path.exists() {
        println!("指定路径存在");
    } else {
        println!("指定路径不存在");
    }
    if path.is_file() {
        println!("指定路径是文件");
    } else if path.is_dir() {
        println!("指定文件是文件夹");
    } else if path.is_symlink() {
        println!("指定文件是连接文件");
    }

    if path.exists() {
        match std::fs::File::open(&PATH) {
            Ok(_) => (),
            Err(_) => {}
        }
    } else {
        match std::fs::File::create(&PATH) {
            Ok(_) => (),
            Err(_) => {}
        }
    }
    let _ = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path);
}

/**
# 读取文件
1. std::fs::read方法直接以字节方式读取路径内容，返回一个`Result<Vec[u8], _>`
    ```rust
    let _data = std::fs::read("data");
    ```
2. 对于打开的文件File类型实例进行读取
    - read_to_string将整个文件读入为字符串
    - read_to_end将整个读入为二进制动态数组
    ```rust
    if let Ok(mut file) = std::fs::File::open("data") {
        // 读入为字符串
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
            println!("{:#?}", data);
        }
    }
    if let Ok(mut file) = std::fs::File::open("data") {
        // 读入为二进制数组
        let mut data = Vec::new();
        if let Ok(_) = file.read_to_end(&mut data) {
            println!("{:#?}", data);
        }
    }
    ```
*/
pub fn f02_read() {
    let _data = std::fs::read("data");
    if let Ok(mut file) = std::fs::File::open("data") {
        // 读入为字符串
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
            println!("{:#?}", data);
        }
    }
    if let Ok(mut file) = std::fs::File::open("data") {
        // 读入为二进制数组
        let mut data = Vec::new();
        if let Ok(_) = file.read_to_end(&mut data) {
            println!("{:#?}", data);
        }
    }
}

/**
# 写入文件
1. std::fs::write方法直接写入字符串到文件中
    ```rust
    match std::fs::write("data", "test") {
        Ok(_) => (),
        Err(e) => println!("{:#?}", e),
    }
    ```
2. 写入或者追加写入需要通过打开方式控制
    ```rust
    let Ok(mut file) = std::fs::OpenOptions::new().append(true).open("data") else {
        println!("打开失败");
        return;
    };
    file.write("abc\n".to_string().as_bytes()).unwrap();
    ```
*/
pub fn f03_write() {
    match std::fs::write("data", "test\n") {
        Ok(_) => (),
        Err(e) => println!("{:#?}", e),
    }
    let Ok(mut file) = std::fs::OpenOptions::new().append(true).open("data") else {
        println!("打开失败");
        return;
    };
    file.write("abc\n".to_string().as_bytes()).unwrap();
    file.write_all("abcdef\n".to_string().as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch19_01() {
        assert_eq!(f01_open_create(), ());
    }

    #[test]
    fn ch19_02() {
        assert_eq!(f02_read(), ());
    }

    #[test]
    fn ch19_03() {
        assert_eq!(f03_write(), ());
    }
}
