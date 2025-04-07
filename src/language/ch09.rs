/*!
# Rust生命周期

- Rust中生命周期就是指引用的有效作用域
- 程序中的变量都存在从创建到内存销毁或者泄漏的过程
- 对于Rust来说一般的变量完全由于语言来管理变量创建销毁以及有效作用域
- 但当使用引用的时候如果编译器推导不出来该引用应该跟着哪个变量就要手动标明生命周期

## 生命周期的主要作用

- 生命周期的主要作用是防止悬垂引用
- 也就是当一个变量被销毁之后其引用就不应该存在
- Rust的生命周期会在编译期做该检查

## 生命周期标识符

- 生命周期标识符是一种标注语法，在引用变量的类型标识符`&`之后添加
- 语法形式是以`'`开头然后通常跟上一个小写字母
- 常见的形式`&'a i32`，`&'b i64`
- 注意生命周期标识符在使用之前都需要在泛型参数列表中声明

## [悬垂指针和生命周期](./fn.f01_dangling_reference.html)

## [函数中的生命周期](./fn.f02_function.html)

## [结构体中的生命周期](./fn.f03_struct.html)

## [方法中的生命周期](./fn.f04_method.html)

## 生命周期消除

- 对于编译器来说，每一个引用都有一个生命周期
- 但是在编程的时候，很多时候不需要标注生命周期标识符
- 编译器有一套消除规则，使得编程的时候不需要标注生命周期标识符
- 编译器能推导出来时明确的生命周期的引用变量，就不需要手动标明生命周期标识符
    1. 消除规则并非万能，若编译器不能确定某件事是正确时，会直接判为不正确，那么还是需要手动标注生命周期
    2. 函数或者方法中，参数的生命周期被称为 输入生命周期，返回值的生命周期被称为 输出生命周期
- 三条消除规则
    1. 每一个引用参数都会获得独自的生命周期
    2. **若只有一个输入生命周期（函数参数中只有一个引用类型），那么该生命周期会被赋给所有的输出生命周期**，也就是所有返回值的生命周期都等于该输入生命周期
    3. 若存在多个输入生命周期，且其中一个为`&self`或者`&mut self`，则`&self`的生命周期赋给所有输出生命周期

## [静态生命周期](./fn.f05_static.html)

## [一个复杂的例子](./fn.f06_case.html)
*/

/**
# 悬垂指针和生命周期
- 悬垂指针，指在变量已经被销毁之后还持有指向该变量的指针
- `A dangling reference is a reference to an object that no longer exists`
- Rust编译器会在编译期做检查，不允许悬垂指针(引用)通过编译

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r); // 导致编译报错，r引用的变量x已经被销毁了
}
```

*/
pub fn f01_dangling_reference() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    }
    // println!("r: {}", r); // 导致编译报错，r引用的变量x已经被销毁了
}

/**
# 函数中的生命周期

- 函数的返回值是引用类型的时候，其生命周期的来源只可能有两个
    1. 参数
    2. 函数体内凭空产生
- 只有来自参数的生命周期才能通过编译，当参数存在多个与返回值类型相同的引用情况下，就需要手动标注生命周期
- 在函数参数中使用生命周期标识符需要在泛型参数列表中先声明生命周期标识符
- 返回函数体内凭空产生的生命周期就是悬垂引用

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
*/
pub fn f02_function() {}

/**
# 结构体中的生命周期

- 结构体字段的类型是引用类型的时候也需要生命周期标注
- 同样需要在结构体的泛型参数列表中需要先声明生命周期标识符

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```
*/
pub fn f03_struct() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let message = "test";

    let i = ImportantExcerpt { part: message };

    println!("{}", i.part);
}

/**
# 结构体中的生命周期

- 来自一般参数的生命周期写在方法的泛型参数列表中
- 来自结构体字段的生命周期写在结构体泛型参数列表中，且在impl块上需要加上泛型参数列表且写明完整类型

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// 应用生命周期消除规则
impl<'a> ImportantExcerpt<'a> {
    // 这个方法的返回值是&str类型，在没有标明参数列表生命周期标识符的情况下，自动跟随`&self`
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 手动标明生命周期标识符
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

*/
#[allow(dead_code)]
pub fn f04_method() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
}

/**
# 静态生命周期

- `'static`生命周期标识符可以让变量活得和程序一样久
- 字符串字面量，提到过它是被硬编码进 Rust 的二进制文件中，因此这些字符串变量全部具有 'static 的生命周期
- 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static，有时候它会给你奇迹

```rust
let s: &'static str = "test";
```
*/
pub fn f05_static() {
    let s: &'static str = "test";
    println!("{s}");
}

/**
# 一个复杂的例子
- 在一个函数中同时包含泛型，生命周期标识符，特征约束
```rust
use std::fmt::Display;
fn longest_with_an_annoucement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
*/
pub fn f06_case() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch09_01() {
        assert_eq!(f01_dangling_reference(), ());
        assert_eq!(f02_function(), ());
        assert_eq!(f03_struct(), ());
        assert_eq!(f04_method(), ());
        assert_eq!(f05_static(), ());
        assert_eq!(f06_case(), ());
    }
}
