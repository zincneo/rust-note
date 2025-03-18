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
    /// ```ignore
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

fn _ch06_test() {
    println!("this is ch06 mod");
}

fn _ch06_02_mod() {
    /// ## 模块
    /// - 使用mod关键字可以定义模块，关键字后面直接跟着模块名
    /// - 模块可以嵌套
    /// - 模块中可以定义各种Rust类型，例如函数，结构体，枚举，特征等等
    /// - 一个模块的所有定义必须在同一个文件内
    /// - 使用模块
    ///   - 绝对路径，以crate为起点
    ///   - 相对路径，以self,super或者当前模块的标识符作为开头
    /// - 当lib.rs中使用mod mode_name声明的时候就可以将模块子模块拆分到src/mode_name.rs或者src/mode_name/mod.rs中
    /// - 模块下的内容默认只对本身以及内部嵌套的子模块可见，模块内部的内容想被外部访问需要加上pub关键字
    ///   - 结构体设置为pub的时候属性和方法任然是外部不可见的，希望外部可以访问还要在属性和方法前面加上pub
    ///   - 枚举设置pub的时候所有字段对外部都是可见的
    fn _mod() {
        #[allow(dead_code)]
        mod front_of_house {
            fn add_to_waitlist() {
                println!("add to wait list");
            }
            // pub关键字让方法被模块外可见
            pub fn seat_at_table() {
                // 绝对路径
                crate::ch06::_ch06_test();
                // 相对路径
                self::add_to_waitlist();
                println!("seat at table");
            }
        }
        front_of_house::seat_at_table();
    }
    _mod();
}

fn _ch06_03_use() {
    /// ## use
    /// - use关键字用来将某个模块中的内容暴露到当前模块下使用
    /// - 多个模块中的内容重名的时候使用use和as关键字来解决冲突
    /// - pub关键字加在use前面表示导入的内容作为当前包的内容再导出的时候设置为可见
    ///   1. pub 意味着可见性无任何限制
    ///   2. pub(crate) 表示在当前包可见
    ///   3. pub(self) 在当前模块可见
    ///   4. pub(super) 在父模块可见
    ///   5. pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块
    /// - 简化导出使用{}
    /// - 使用*使用mod下的所有项
    #[allow(unused)]
    fn _use() {
        // as解决冲突
        use std::fmt::Result;
        use std::io::Result as IoResult;
        mod front_of_house {
            pub mod hosting {
                pub fn add_to_waitlist() {}
            }
        }
        // 在当前mod使用hosting并且设置再次导出的时候被外部包可见
        pub use front_of_house::hosting;
        mod a {
            pub fn aa() {}
            pub fn ab() {}
        }
        pub use a::{aa, ab};
        mod b {
            pub fn ba() {}
            pub fn bb() {}
        }
        pub use b::*;
    }
    _use();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch06_01() {
        assert_eq!(_ch06_01_project(), ());
    }

    #[test]
    fn ch06_02() {
        assert_eq!(_ch06_02_mod(), ());
    }

    #[test]
    fn ch06_03() {
        assert_eq!(_ch06_03_use(), ());
    }
}
