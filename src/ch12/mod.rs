/*!
# 包和模块级别的注释
- 除了文档行、块注释提供给Rust结构体以及函数之外，还可以提供包和模块级别的注释
- 必须放在整个包或者模块的开头
*/

/// ## 文档行注释
/// - rust提供了`cargo doc`命令可以将文档注释转换为HTML网页
/// - 使用///表示文档行注释
/// - 文档注释必须位于lib类型的crate当中
/// - 文档注释可以使用markdown语法
/// - 文档注释的对象必须是pub对外可见的
fn _ch12_01_doc_comment() {}

/**
## 文档块注释
*/
fn _ch12_02_doc_block_comment() {}

/// ## 文档测试
/// - 文档注释中的代码块会被`cargo test`的时候运行作为单元测试
/// - 只想运行文档测试的话使用`cargo test --doc`
/// - 不想运行的代码块需要标明ignore
/// ```rust
/// let arg = 5;
/// let answer = rust_note::ch12::add_one(arg);
/// assert_eq!(6, answer);
/// ```
///
/// ```ignore
/// // 忽略这个代码块
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch12_01() {
        assert_eq!(_ch12_01_doc_comment(), ());
    }

    #[test]
    fn ch12_02() {
        assert_eq!(_ch12_02_doc_block_comment(), ());
    }
}
