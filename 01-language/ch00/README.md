# ch01 Rust语言简介

## 语言特点

1. Rust是强类型的编译型语言
2. Rust严格且强大的编译器在编译期可以避免大部分错误
3. Rust无GC且无需手动管理内存
4. Rust运行时效率与c/c++同一级别
5. Rust提供优秀的官方项目管理工具(包管理器)cargo

## 最简单的Rust程序

1. Rust程序的扩展名为`.rs`
2. 官方提供的基于llvm后端编译器的rustc
3. 编译一个最小的可执行程序`rustc main.rs`
4. 一个Rust可执行程序的必须要有main函数，这是整个程序的入口

```rust
fn main() {
    println!("Hello Rust!");
}
```

## 包管理器cargo

1. cargo命令是rust语言官方提供的项目管理工具
2. 项目依赖于文件夹下的Cargo.toml来配置项目和管理第三方依赖，Cargo.lock来锁定包的版本
3. 创建项目
    - 二进制项目`cargo new project_name`
    - 库项目`cargo new --lib project_name`
5. 运行项目
    - debug模式`cargo run`
    - release模式`cargo run --release`
5. 运行测试用例
    - 项目所有测试用例`cargo test`
    - 指定模块下的测试用例`cargo test mod_name::test_fn`
    - 文档测试`cargo test --doc`
