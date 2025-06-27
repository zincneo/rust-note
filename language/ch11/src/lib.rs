/*!
# Rust包和模块

当工程规模变大时，把代码写到一个甚至几个文件中，都是不太聪明的做法，可能存在以下问题：

1. 单个文件过大，导致打开、翻页速度大幅变慢
2. 查询和定位效率大幅降低，类比下，你会把所有知识内容放在一个几十万字的文档中吗？
3. 只有一个代码层次：函数，难以维护和协作，想象一下你的操作系统只有一个根目录，剩下的都是单层子目录会如何：disaster
4. 容易滋生 Bug

同时，将大的代码文件拆分成包和模块，还允许我们实现代码抽象和复用：将你的代码封装好后提供给用户，那么用户只需要调用公共接口即可，无需知道内部该如何实现。

Rust 提供了相应概念用于代码的组织管理：

1. 项目(Package): 一个`Cargo`提供的`Feature`，可以用来构建、测试和分享包
2. 包(Crate): 一个由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件进行运行
3. 模块(Module)：可以一个文件多个模块，也可以一个文件一个模块，模块可以被认为是真实项目中的代码组织单元

## Package

- Package就是一个项目，有一个`Cargo.toml`配置管理
- 一个Package可以包含一个`lib`类型的Crate以及多个二进制Crate
- 创建二进制Crate`cargo new package_name`
- 创建库Crate`cargo new --lib package_name`
- Package容易和Crate混淆
    - `cargo new`创建的时候Package和包含的Crate是同名的
- 典型的Package结构
```
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
```
- 唯一的库Crate: `src/lib.rs`
- 默认的二进制Crate: `src/main.rs`，编译后生成的可执行程序和Package同名
- 其他二进制Crate: `bin/?.rs`，编译后生成和文件同名的可执行程序
- 集成测试文件: `tests/`目录下
- 基准性能测试: `benches`目录下
- 项目示例: `examples`目录下
*/

mod m01_module;
mod m02_visible;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_module() {
        assert_eq!(m01_module::f01_mod(), ());
    }

    #[test]
    fn test02_visiable() {
        assert_eq!(m02_visible::f01_visible(), ());
        assert_eq!(m02_visible::f02_use(), ());
    }
}
