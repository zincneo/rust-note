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
pub fn f01_generics() {
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
pub fn f02_const_generics() {
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
