# Cargo

cargo是Rust官方提供的命令行包管理工具

## cargo的功能

1. 管理依赖
2. 简化编译过程
3. 项目构建和管理
4. 测试和文档
5. 发布和版本控制
6. 跨平台构建

## rustc

Rust的底层编译器，就像gcc/clang对于c++的作用相同，编译rs文件生成库文件或者可执行文件

## rustup

终端Rust环境维护命令，可以安装不同版本的Rust编译器、标准库、扩展模块

```sh
rustup install nightly # 安装预览版本
rustup install stable # 安装稳定版本
rustup default nightly # 指定全局默认为预览版本
rustup override set nightly # 指定当前项目为预览版本
```
