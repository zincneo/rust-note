#![allow(dead_code)]
#![allow(unused)]

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_path() {
        assert_eq!(f01_path(), ());
    }
}
