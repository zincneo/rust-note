/*!
# Rust注释和文档

在Rust中，注释分为三类:
1. 代码注释，用于说明某一块代码的功能，读者往往是同一个项目的协作开发者
2. 文档注释，支持 Markdown，对项目描述、公共 API 等用户关心的功能进行介绍，同时还能提供示例代码，目标读者往往是想要了解你项目的人
3. 包和模块注释，严格来说这也是文档注释中的一种，它主要用于说明当前包和模块的功能，方便用户迅速了解一个项目

## 代码注释

1. 行注释`// ...`
2. 块注释`/* ... */`

## 文档注释

- 文档注释需要位于`lib`类型的crate中，例如`src/lib.rs`
- 文档注释支持Markdown语法
- 被注释的对象需要使用 pub 对外可见，记住：文档注释是给用户看的，内部实现细节不应该被暴露出去
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

- 跳转到标准库
```rust
/// `add_one` 返回一个[`Option`]类型
pub fn add_one(x: i32) -> Option<i32> {
    Some(x + 1)
}
```

- 使用完整路径跳转到指定项目

```rust
pub mod a {
    /// `add_one` 返回一个[`Option`]类型
    /// 跳转到[`crate::MySpecialFormatter`]
    pub fn add_one(x: i32) -> Option<i32> {
        Some(x + 1)
    }
}

pub struct MySpecialFormatter;
```

- 遇到同名项的情况下指定跳转

```rust
/// 跳转到结构体  [`Foo`](struct@Foo)
pub struct Bar;

/// 跳转到同名函数 [`Foo`](fn@Foo)
pub struct Foo {}

/// 跳转到同名宏 [`foo!`]
pub fn Foo() {}

#[macro_export]
macro_rules! foo {
  () => {}
}
```
*/
