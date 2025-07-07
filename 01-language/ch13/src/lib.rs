/**
# 注释

## 代码注释
1. `// ...` 行注释
2. `/* ... */` 块注释

## 文档注释

- 文档注释需要位于`lib`类型的crate中，例如`src/lib.rs`
- 文档注释支持Markdown语法
- 被注释的对象需要使用 pub 对外可见，记住：文档注释是给用户看的，内部实现细节不应该被暴露出去
- 可以用于函数、结构体、枚举、const常量等等
- 文档行注释`/// ...`
- 文档块注释`/** ... */`
- 生成并查看文档的命令`cargo doc --open`

## 包和模块注释

- 这种注释需要添加到模块或者crate的最上方
- 行注释`//! ...`
- 块注释`/*! ... */`

## 文档测试

- 命令`cargo test --doc`
- 在文档注释中使用Markdown代码块会作为文档测试
    1. 代码块标记为rust,should_panic表示这个用例必须panic
    2. 代码块中使用#标记的行会在文档中隐藏，但是运行文档测试还是会运行

## 文档跳转
- 在文档注释中使用`[`标准库导出内容`]`可以调整到标准库
- 在文档注释中使用`[`绝对路径/相对路径`]`
- 存在同名项目的时候指定跳转
    - 跳转到结构体`[`Foo`](struct@Foo)`
    - 跳转到函数`[`Foo`](fn@Foo)`
    - 跳转到宏`[`Foo!`]`
*/
pub fn f01_comment() {
    /// `add_one` 返回一个[`Option`]类型
    pub fn add_one(x: i32) -> Option<i32> {
        Some(x + 1)
    }

    add_one(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_comment() {
        assert_eq!(f01_comment(), ());
    }
}
