/*!
# Rust 宏编程

1. 声明式宏(declarative macro) `macro_rules!`
2. 过程式宏(procedural macros)
    1. `#[derive]` 派生宏
    2. 类属性宏，用于为目标添加自定义属性
    3. 类函数宏，看上去像是函数调用

## 宏和函数的差异

1. 元编程 - 宏本质上是通过一种代码生成另一种代码，因此是元编程的一种
2. 可变参数 - Rust函数签名是固定的，而宏可以拥有可变数量参数
3. 宏展开 - 宏可以为指定的类型实现某个特征：先将宏展开成实现特征的代码后，再被编译
*/

/**
# 声明式宏

1. 使用关键词`macro_rules!`定义
2. 类似match，在宏的代码块中进行分支匹配
3. `#[macro_export]`注释将宏导出，其他包才可以索引到该宏
4. 该特性将来可能会被弃用
5. 代码中展示一个简化的vec!示例来演示如何在代码块中做模式匹配
    - 解释模式`( $( $x:expr ),* )`
        1. `$()`中包含的是模式 `$x:expr`，该模式中的 expr 表示会匹配任何 Rust 表达式，并给予该模式一个名称 `$x`
        2. `$()` 之后的逗号说明 `$()` 所匹配的代码使用逗号分隔符分割，紧随逗号之后的 * 说明 * 之前的模式($()内的部分)会被匹配零次或任意多次(类似正则表达式)(且以逗号分割)
        3. 注意该写法`cvec![1, 2, 3]`合法但是`cvec![1, 2, 3,]`不合法，如果要支持模式应该写为`($($x:expr),+ $(,)?)`
*/
pub fn macros_by_example() {
    macro_rules! cvec {
        ( $( $x:expr ),* ) => {
            {
                let mut tmp_vec = Vec::new();
                $(
                    tmp_vec.push($x);
                )*
                tmp_vec
            }
        };
    }

    let v = cvec!(1, 2, 3, 4);
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_macros_by_example() {
        assert_eq!(macros_by_example(), ());
    }
}
