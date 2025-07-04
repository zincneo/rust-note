#![allow(dead_code)]
#![allow(unused)]

use std::io::{Read, Write};

/**
# path模块
- 提供Path、PathBuf两个类用来处理文件路径
- Path是不可变的路径切片(类似str)
- PathBuf是可变、拥有所有权的路径(类似String)
- 经常和&str、String互相转换
- 常用方法
    - file_name 获取文件名
    - parent 获取父目录
    - extension 获取扩展名
    - has_root 判断是否有根路径
    - is_absolute 是否是绝对路径
    - is_relative 是否是相对路径
    - canonicalize 简化路径
    - strip_prefix 剥离前缀
    - is_file 是否是文件
    - is_dir 是否是文件夹
    - is_symlink 是否是连接文件
*/
pub fn f01_path() {
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};
    let path = Path::new("/tmp/foo.txt");

    let mut path_buf = PathBuf::new();
    path_buf.push("/tmp");
    path_buf.push("bar.txt");
    let path = Path::new("/home/user/docs/file.txt");

    // 获取文件名（含扩展名）
    assert_eq!(path.file_name(), Some(OsStr::new("file.txt"))); // 注意返回 OsStr

    // 获取父目录
    assert_eq!(path.parent(), Some(Path::new("/home/user/docs")));

    // 获取扩展名
    assert_eq!(path.extension(), Some(OsStr::new("txt")));

    // 判断是否有根路径
    assert_eq!(path.has_root(), true);

    // 路径遍历
    for component in path {
        println!("{:?}", component); // "/ home user docs file.txt"
    }

    // 路径拼接
    // Path是不可变类型，但是有join方法来获取一个新的PathBuf
    let base = Path::new("/tmp");
    // 自动处理分隔符
    let full_path = base.join("data/logs/app.log");

    // 检查路径属性
    path.is_absolute(); // 是否绝对路径
    path.is_relative(); // 是否相对路径
    path.exists(); // 需配合 std::fs

    // 常用方法
    {
        // 获取当前工作目录
        let cwd = std::env::current_dir(); // 返回 PathBuf

        // 规范化路径（消除冗余组件）
        let p = Path::new("/foo/.././bar").canonicalize(); // -> /bar

        // 路径剥离前缀
        let rel_path = Path::new("/home/user/file.txt").strip_prefix("/home"); // -> user/file.txt
    }
}

/**
# 文件读写
- std::fs模块提供文件类型File
- 打开/创建文件
    - `File::open`、`File::create`、`File::create_new`
    - 都返回Result<File, Error>
    - 离开作用域文件自动关闭，变量被丢弃
    - open以可读模式打开
    - create以可写模式创建或打开文件
    - create_new在文件存在时会返回Err
- std::fs模块提供访问控制类OpenOptions
    - 提供write/read/append三个方法来控制打开时的权限
    - 提供open方法打开指定路径的文件
- 读取文件内容
    - std::fs::read方法直接以字节方式读取路径内容，返回一个`Result<Vec[u8], _>`
    - File类型的读取方法
        - read_to_string将整个文件读入为字符串
        - read_to_end将整个读入为二进制动态数组
- 写入文件内容
    - std::fs::write方法直接写入字符串到文件中
    - 写入或者追加写入需要通过打开方式控制
*/
fn fn02_fs() {
    use std::fs;
    use std::path::Path;
    let path = Path::new("foo.txt");
    {
        let file = fs::File::open(path);
    }
    {
        let file = fs::File::create(path);
    }
    {
        let file = fs::File::create_new(path);
    }
    // 指定打开时的权限
    {
        let file = fs::OpenOptions::new().read(true).write(true).open(path);
    }
    // 读取文件内容
    {
        let data = fs::read(path);
        if let Ok(mut file) = fs::File::open(path) {
            let mut data = String::new();
            let size = file.read_to_string(&mut data);
        }
        if let Ok(mut file) = fs::File::open(path) {
            let mut data = Vec::new();
            let size = file.read_to_end(&mut data);
        }
    }
    // 写入文件内容
    {
        fs::write(path, "test");
        if let Ok(mut file) = fs::OpenOptions::new().write(true).open(path) {
            file.write("test".as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_path() {
        assert_eq!(f01_path(), ());
    }

    #[test]
    fn test02_fs() {
        assert_eq!(fn02_fs(), ());
    }
}
