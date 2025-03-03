fn _ch06_01_project() {
    /// ## 项目工程化管理
    /// 1. Rust通常使用命令行项目管理工具Cargo来管理工程
    /// 2. 由Cargo提供的特性Package(项目)，可以用来构建、测试和分包
    ///   - 包含有一个独立的Cargo.toml文件以及因为功能性被组织在一起的一个或者多个包
    ///   - 一个Package只能包含一个库(libraray)类型的包，但是可以包含多个二进制类型可执行的包
    ///   - 创建一个二进制的Package `cargo new my-project`
    ///     - `src/main.rs`是二进制包的根文件
    ///   - 创建一个库类型的Package `cargo new --lib my-lib`
    ///     - `src/lib.rs`是库文件的根文件
    ///   - 注意Package的名字和创建的包的名字是完全相同的
    /// 3. 包(Crate): 一个由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件进行运行
    ///     - 对于 Rust 而言，包是一个独立的可编译单元，它编译后会生成一个可执行文件或者一个库
    /// 4. 模块(Module): 可以一个文件多个模块，也可以一个文件一个模块，模块可以被认为是真实项目中的代码组织单元
    /// ### 常见的Package目录结构
    /// ```
    /// .
    /// ├── Cargo.toml
    /// ├── Cargo.lock
    /// ├── src
    /// │   ├── main.rs
    /// │   ├── lib.rs
    /// │   └── bin
    /// │       └── main1.rs
    /// │       └── main2.rs
    /// ├── tests
    /// │   └── some_integration_tests.rs
    /// ├── benches
    /// │   └── simple_bench.rs
    /// └── examples
    ///     └── simple_example.rs
    /// ```
    /// - 唯一的库包:src/lib.rs
    /// - 默认二进制包:src/main.rs
    /// - 其余二进制包:src/bin/main1.rs和src/bin/main2.rs，分别会生成一个和文件同名的二进制可执行文件
    /// - 集成测试文件: tests目录下
    /// - 基准性能测试文件: benches目录下
    /// - 项目示例: examples文件夹下
    fn project() {}
    project();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch06_01() {
        assert_eq!(_ch06_01_project(), ());
    }
}
