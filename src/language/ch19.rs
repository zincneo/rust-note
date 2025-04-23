/*!
# Rust读写文件

1. [打开/创建文件](./fn.f01_open_create.html)
2. 读取文件
3. 写入文件
*/

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch19_01() {
        assert_eq!(f01_open_create(), ());
    }
}
