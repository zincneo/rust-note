/*!
# ch07 Rust泛型和特征

## 00 多态和泛型

在编程中，经常有这样的需求：用同一功能的函数处理不同类型的数据，例如两个数的加法，无论是整数还是浮点数，甚至是自定义类型，都能进行支持。在不支持泛型的编程语言中，通常需要为每一种类型编写一个函数：

```rust
fn add_i8(a:i8, b:i8) -> i8 {
    a + b
}
fn add_i32(a:i32, b:i32) -> i32 {
    a + b
}
fn add_f64(a:f64, b:f64) -> f64 {
    a + b
}
```

上述代码可以正常运行，但是很啰嗦，如果你要支持更多的类型，那么会更繁琐。

解决上述问题就要使用到多态，通俗的讲，多态就是好比坦克的炮管，既可以发射普通弹药，也可以发射制导炮弹（导弹），也可以发射贫铀穿甲弹，甚至发射子母弹，没有必要为每一种炮弹都在坦克上分别安装一个专用炮管，即使生产商愿意，炮手也不愿意，累死人啊。所以在编程开发中，也需要这样“通用的炮管”，这个“通用的炮管”就是多态。

实际上，泛型就是一种多态。泛型主要目的是为程序员提供编程的便利，减少代码的臃肿，同时可以极大地丰富语言本身的表达能力，为程序员提供了一个合适的炮管。

使用泛型实现上面的函数

```rust
// 并不能通过编译，简单看个概念，后面还要学习特征约束，让T泛型被约束保证+运算符可以工作
fn add<T>(a:T, b:T) -> T {
    a + b
}
```

## 01 泛型

1. [泛型参数](./fn.f01_01_generics.html)
2. [const泛型](./fn.f01_02_const_generics.html)

## 02 特征

以文件系统举例，文件操作主要包含四个：open 、write、read、close，这些操作可以发生在硬盘，可以发生在内存，还可以发生在网络 IO，如果要为每一种情况单独实现一套代码就过于复杂了。将这些方法抽象出来，在Rust中就要使用特征trait这一概念。在其他的编程语言当中可能会通过继承接口类的方式实现(java，c++)，本质上都是多态的表现形式，即不同的类型具有相同的行为。

1. [特征定义和实现](./fn.f02_01_trait.html)
2. [使用特征](./fn.f02_02_use_trait.html)

*/

/**
## 泛型参数

- 函数的泛型参数列表在函数名之后`fn function<T, U>()`
    - 泛型参数命名可以随便起，T(Type)经常作为泛型参数名
    - 调用的时候可以显示指定类型`function::<具体的类型>()`(在编译器推导不出来的时候必须写)

```rust
fn test<T>() {}
// 编译器推导不出来的时候必须写明泛型参数值
test::<i32>();
```

- 结构体的泛型参数列表在结构体名之后
    - 在定义的时候可以显示指定类型

```rust
struct Point<T> {
    x: T,
    y: T,
}
let p = Point::<i128> { x: 100, y: 0 };
println!("{}{}", p.x, p.y);
```

- 枚举的泛型参数列表在枚举名之后

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- 方法使用泛型必须在impl和(结构体/枚举)名之后都加上泛型参数列表
    - 并且方法名后可以有自己的泛型参数列表

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // 方法可以自己有泛型参数列表
    fn test<U>(&self) {}
}
```

- 可以为具体类型实现方法

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
*/
pub fn f01_01_generics() {
    struct Point<T> {
        x: T,
        y: T,
    }
    let p = Point::<i128> { x: 100, y: 0 };
    println!("{}{}", p.x, p.y);
}

/**
# const泛型
- 普通泛型针对类型，const泛型针对值
- const泛型语法`<const VAR: type>`
```rust
fn test<T, const N: usize>(arr: [T; N]) {}
```
- const fn 常量函数
    - 在编译器执行这些函数，将计算结果直接嵌入到生成的代码中
    - 目的是提高运行时性能
    - 只需要在函数声明的前面加上const关键字

```rust
const fn add(a: usize, b: usize) -> usize {
    a + b
}
```

- const泛型和const fn结合使用

```rust
struct Buffer<const N: usize> {
    data: [u8; N],
}
const fn compute_buffer_size(factor: usize) -> usize {
    factor * 1024
}
const SIZE: usize = compute_buffer_size(4);
let buffer = Buffer::<SIZE> { data: [0; SIZE] };
println!("Buffer size: {} bytes", buffer.data.len());
```
*/
pub fn f01_02_const_generics() {
    struct Buffer<const N: usize> {
        data: [u8; N],
    }
    const fn compute_buffer_size(factor: usize) -> usize {
        factor * 1024
    }
    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> { data: [0; SIZE] };
    println!("Buffer size: {} bytes", buffer.data.len());
}

/**
# 特征定义和实现

- 定义特征使用`trait TraitName {}`
```rust
trait Summary {
    fn summarize(&self) -> String;
}
```
- 实现特征使用`impl TraitName for StructOrEnumName {}`
```rust
struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}
```
- 在定义特征时可以提供方法的默认实现
    - 在实现特征的时候可以使用默认的也可以选择重载特征中的方法
```
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
// 使用默认实现
impl Summary for Post {}
// 重载方法
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
```

- 为类型A实现特征T，那么A和T至少有一个在当前作用域中定义
    - 该规则称为**孤儿规则**，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码

*/
#[allow(unused)]
#[allow(dead_code)]
pub fn f02_01_trait() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub struct Post {
        pub title: String,   // 标题
        pub author: String,  // 作者
        pub content: String, // 内容
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }
}

/**
# 使用特征
- 作为函数的参数
```rust
trait Test {
    fn test(&self) {
        println!("test");
    }
}
struct A;
struct B;
impl Test for A {}
impl Test for B {}
fn test(obj: &impl Test) {
    obj.test();
}
let (a, b) = (A, B);
test(&a);
test(&b);
```

- 特征约束
    - impl TraitName本质上就是特征约束的语法糖
    - 特征约束语法`<T: TraitName>`

```rust
fn test(obj: &impl Test) {}
// 等价于使用特征约束
fn test<T: Test>(obj: &T) {}
```

- 多重约束
    - 可以使用+运算符表示被多个特征约束
    - 语法糖形式`fn notify(item: &(impl Summary + Display)) {}`
    - 特征约束形式`fn notify<T: Summary + Display>(item: &T) {}`

- where特征约束
    - 特征约束比较复杂的时候不想让函数签名过于复杂可以使用where

```rust
fn some_function_v1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// 等价于
fn some_function_v2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

- 使用特征约束可以有条件地实现方法

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 任意泛型参数的Pair都有new方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// 泛型参数实现约束特征的时候才可以使用的方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

- 函数返回中的impl TraitName
    - 注意这种方式编译器只能推导出确定的唯一类型，并不能真的返回不同类型的值
    - 如果要返回不同类型的值则需要使用特征对象
    - 用途是如果返回值类型过于复杂不好声明(例如闭包和迭代器)可以使用

```rust
// 只能推导出确定的唯一类型，返回值类型容易写的就没必要这么用了
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}
```

- 调用方法需要引入特征
    - 如果要使用某一特征的方法，那么需要在当前作用域引入对应的特征

```rust
use std::convert::TryInto;

fn main() {
  let a: i32 = 10;
  let b: u16 = 100;

  // try_into方法属于TryInto特征，要引入特征才可以使用
  let b_ = b.try_into()
            .unwrap();

  if a < b_ {
    println!("Ten is less than one hundred.");
  }
}
```

- 通过实现特征来重载运算符

```rust
use std::ops::Add;

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}
```
*/
pub fn f02_02_use_trait() {
    trait Test {
        fn test(&self) {
            println!("test");
        }
    }
    struct A;
    struct B;
    impl Test for A {}
    impl Test for B {}
    fn test(obj: &impl Test) {
        obj.test();
    }
    let (a, b) = (A, B);
    test(&a);
    test(&b);

    use std::ops::Add;

    #[derive(Debug)]
    struct Point<T>
    where
        T: Add<T, Output = T>,
    {
        x: T,
        y: T,
    }
    impl<T> Add for Point<T>
    where
        T: Add<T, Output = T>,
    {
        type Output = Point<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", p1 + p2);

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", p3 + p4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch07_01() {
        assert_eq!(f01_01_generics(), ());
        assert_eq!(f01_02_const_generics(), ());
    }

    #[test]
    fn ch07_02() {
        assert_eq!(f02_01_trait(), ());
        assert_eq!(f02_02_use_trait(), ());
    }
}
