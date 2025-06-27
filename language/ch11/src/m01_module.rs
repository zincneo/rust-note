#![allow(dead_code)]
#![allow(unused)]

mod custom {
    pub fn test() {
        println!("test");
    }

    pub mod inner {
        pub fn inner() {
            println!("innner");
        }
    }
}

/**
# 模块

- 模块使用`mod关键字进行定义`
- 模块中可以嵌套定义模块
- 模块命名规则同变量名
- 模块中可以定义各种Rust类型，例如函数、结构体、枚举、特征等
- 所有模块定义均在一个文件中
- 模块树
    - `src/main.rs`和`src/lib.rs`称为(crate root)，以上的示例形成如下的模块树
    - 使用模块树中的内容需要使用操作符`::`
- 用路径引用模块
    - 绝对路径，从`crate root`开始，路径名为`crate_name::`或者`crate::`
    - 相对路径，从当前所在的模块开始`self::`或者父模块开始`super::`
- 在当前文件中导入模块
    - `mod mod_name;`
    - 可以给定文件为同文件夹下的`mod_name.rs`或者同文件夹下的`mod_name/mod.rs`
*/
pub fn f01_mod() {
    // 1. 相对路径
    // 当前代码所在的模块开始
    custom::test();
    self::custom::test();
    super::m01_module::custom::test();

    // 2. 相对路径
    // 从crate root开始
    crate::m01_module::custom::inner::inner();
}
