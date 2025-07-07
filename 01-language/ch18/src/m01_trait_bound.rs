#![allow(dead_code)]
#![allow(unused)]

/**
# 特征约束
- 在使用泛型的时候，如果希望类型受到一定限制，那么就可以使用特征约束语法来限制泛型参数
- `fn f<T: Display>(t: T)`这里就对于T类型进行了限制，编译器会检查调用时传入的实参是否实现了Display特征
- 之前在泛型章节中见过的`impl TraitName`是特征约束的语法糖`fn f() -> impl TraitName`等价于`fn f<T: TraitName>() -> T`
*/
pub fn f01_trait_bound() {
    use std::fmt::{Debug, Display};
    fn f<T: Display>(t: T) {
        println!("{}", t);
    }
    f(10);
    f("str");
    // 用于结构体
    #[derive(Debug)]
    struct Custom<T: Debug, U: Debug> {
        t: T,
        u: U,
    }
    let custom = Custom { t: 60.0, u: "str" };
}

/**
# 多重约束
- 在使用特征约束语法的时候`TraitName`之间可以使用`+`表示多重约束，即被约束的类型必须同时实现这些特征
*/
pub fn f02_multiple_bound() {
    use std::fmt::{Debug, Display};

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }
}

/**
# where子句
- 约束不一定要写在泛型参数列表中，也可以在结构体或者是函数以及impl块的`{}`之前使用where子句来进行约束
*/
pub fn f03_where() {
    use std::fmt::Debug;
    // 1. 函数
    fn f<T>()
    where
        T: Debug,
    {
    }
    // 2. 结构体
    struct Custom<T>
    where
        T: Debug,
    {
        t: T,
    }
    // 3. impl 块
    impl<T> Custom<T> where T: Debug {}
}
