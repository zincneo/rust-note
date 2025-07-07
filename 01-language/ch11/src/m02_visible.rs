#![allow(dead_code)]
#![allow(unused)]

struct Custom {
    id: String,
}
/**
# 代码可见性
- 同一模块内的代码不存在可见性限制
- 不同模块之间存在可见性限制
    1. 模块内的内容默认对外部不可见:函数、结构体、枚举、方法、特征、静态变量、模块
    2. 父模块无法访问子模块中的内容
    3. 子模块可以无条件访问父模块和更上层的直接祖父模块中的内容
- pub关键字
    - pub关键字修饰可以让外部可以访问模块中的内容
    - pub关键字可以修饰:函数、结构体、结构体字段、枚举、方法、特征、静态变量、模块
    - pub修饰mod，可以让模块在更上一层的模块可见
    - pub修饰struct，只能让结构体可见，字段需要额外设置可见性
    - pub修饰enum，所有枚举值对外可见
    - pub可以精确控制可见范围
        - pub表示无条件可见
        - pub(crate)表示在当前crate中可见
        - pub(self)表示在当前模块可见
        - pub(super)表示在父模块中可见
        - pub(in <path>)表示在某个路径中可见，路径`path`必须是父模块或者祖先模块
*/
pub fn f01_visible() {
    mod outter {

        struct Custom {
            id: String,
        }

        pub struct Person {
            name: String,
        }

        impl Person {
            fn new(name: String) -> Self {
                Self { name }
            }
            pub fn echo(&self) {
                println!("{:?}", self.name);
            }
        }

        mod inner_1 {
            pub fn test() {
                // 1. 父模块中的内容对当前模块可见，使用相对路径进行访问
                let custom = super::Custom {
                    id: "0".to_string(),
                };
                // 2. 父模块中的内容对当前模块可见，使用绝对路径访问
                let custom = crate::m02_visible::Custom {
                    id: "1".to_string(),
                };
            }
        }

        pub mod inner_2 {
            // 3. 通过可见模块中的可见函数返回外部可见的结构体实例
            pub fn person() -> super::Person {
                super::Person::new("zinc".to_string())
            }
        }
    }

    let p = outter::inner_2::person();
    p.echo();
}

/**
# use
- use关键字的作用是用来简化路径和再导出
- `use EnumName`使得当前作用域可以使用枚举类型下的所有枚举值
- `use mod_name::func_name`使得当前作用域下可以`func_name`而不需要通过`mod_name::func_name`访问
- `pub use mod_name::func_name`等价于再导出，让上级模块认为当前该模块有可见的`func_name`
- as关键字可以给用来起别名`use mod_name::func_name as func`
- `{}`简化导出语句`use A::a; use A::b;` == `use A::{a, b};`
- `use mod_name::*`表示导出模块下所有的可见项
*/
pub fn f02_use() {
    mod outter {
        pub mod inner {
            pub fn test() {
                println!("test");
            }
        }
        // 1. 以别名再导出
        pub use inner::test as tt;
        pub enum Enum {
            A,
            B,
            C,
            D,
        }
    }
    outter::inner::test();
    outter::tt();
    use outter::Enum::*;
    let t = A;
}
