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

## Crate

- 对于Rust而言，Crate是一个独立的可编译单元，它编译后会产生一个可执行文件或一个库
- 一个包会将相关联的功能打包在一起，使得该功能可以很方便的在多个项目中分享
    - 标准库中没有提供但是在三方库中提供的`rand`包举例
    - 这个包提供了随机数生成的功能
    - 只需要将该包通过`use rand;`引入到当前项目的作用域中，就可以在项目中使用`rand`的功能:`rand::XXX`
    - 同一个包中不能有同名的类型，但是在不同包中就可以
    - `rand`包中，有一个`Rng`特征，依然可以在自己的项目中定义一个`Rng`

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

## [模块Module](./mod_case/index.html)

## [代码可见性](./mod_visiable/index.html)

## 模块与文件分离

- 在当前文件中声明的模块`mod mod_name {}`可以拆分为
    1. 在当前文件夹下创建文件`mod_name.rs`，然后在原文件中使用模块`use mod_name;`
    2. 在当前文件夹下创建文件`mod_name/mod.rs`，然后在原文件中使用模块`use mod_name;`

## [use关键字和受限可见性](./mod_use/index.html)
*/

/**
# 模块

- 模块使用`mod关键字进行定义`
- 模块中可以嵌套定义模块
- 模块中可以定义各种Rust类型，例如函数、结构体、枚举、特征等
- 所有模块定义均在一个文件中
```rust
// 餐厅前厅，用于吃饭
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

- 模块树
    - `src/main.rs`和`src/lib.rs`称为(crate root)，以上的示例形成如下的模块树
    - 使用模块树中的内容需要使用操作符`::`
```rust
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

- 用路径引用模块
    - 绝对路径，从`crate root`开始，路径名为`crate_name::`或者`crate::`
    - 相当路径，从当前所在的模块开始`self::`或者父模块开始`super::`

```rust
fn serve_order() {
    // 从当前模块开始使用
    self::back_of_house::cook_order()
    // 等价于
    back_of_house::cook_order()
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 从父模块开始使用
        super::serve_order();
    }

    fn cook_order() {}
}
```
*/
pub mod mod_case {}

/**
# 代码可见性
- 同一模块内的代码不存在可见性问题
- 对于模块外部，不同模块之间具有可见性设置
- 模块内的内容默认都是私有的:包括函数、结构体、枚举、方法、特征、常量以及嵌套在内部的模块
- 父模块无法访问子模块的私有内容，但是子模块可以访问父模块，父父..模块的私有项
- pub关键字
    - pub关键字用来指定对于模块之外的可见性
    - pub作用于结构体的时候，成员属性任然是私有的，如果希望成员属性被外部可见需要对成员属性单独也加上pub
    - pub作用于枚举的时候，枚举值均为可见
```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}
// 这里可以使用front_of_house::hosting
// 但是不能使用front_of_house::hosting::add_to_waitlist
```
*/
pub mod mod_visiable {}

/**
# use关键字和受限可见性

- use关键字用来简化路径
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 绝对路径
use crate::front_of_house::hosting;
// 相对路径
// use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

```

- 使用use的时候注意避免造成同一作用域下出现同名项即可
- as关键字可以在use引用的时候重新命名

```rust
use std::fmt::Result;
// 通过as关键字避免出现同名项
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

- 利用use关键字将模块在当前模块再导出
    - 当外部的模块项 A 被引入到当前模块中时，它的内容可见性自动被设置为私有的
    - 再导出的时候希望被外部可见需要加上pub关键字

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// 当前模块再导出hosting
// 默认情况下hosting内容再当前是私有的
// 加上pub关键字外部才能通过hosting::add_to_waitlist访问
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- 使用`{}`简化引用

```rust
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

use std::cmp::Ordering;
use std::io;
use std::io::Write;
```

```rust
use std::collections::{HashMap,BTreeMap,HashSet};
use std::{cmp::Ordering, io::{self, Write}};
```

- 使用`*`可以引用模块下的所有项
- 通过`pub()`来控制可见范围
    - pub表示无条件可见
    - pub(crate)表示在当前crate中可见
    - pub(self)表示在当前模块可见
    - pub(super)表示在父模块中可见
    - pub(in <path>)表示在某个路径中可见，路径`path`必须是父模块或者祖先模块
*/
pub mod mod_use {}
