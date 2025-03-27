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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch07_01() {
        assert_eq!(f01_01_generics(), ());
        assert_eq!(f01_02_const_generics(), ());
    }
}
