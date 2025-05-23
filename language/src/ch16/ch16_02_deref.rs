/*!
# Deref

- Rust提供了`*`运算符用来解引用
- Rust中存在引用类型自动转换的行为
- 发生该行为有以下要求
    - 函数的实参匹配形参的时候
    - 通过`.`运算符进行方法调用时，调用者会发生引用类型自动转换
    - 发生自动转换的变量必须是实现了特征Deref的类型

## [1. `*`解引用](./fn.f02_01_star_mark.html)

## [2. 自动解引用行为](./fn.f02_02_auto_deref.html)

## [3. 手动实现Deref特征](./fn.f02_03_deref_impl.html)

## 4. 引用归一化

- 编译器实质上只会做&v形式的自动解引用，如果是智能指针或者是`&&&&v`类型，会以链式调用的形式触发Deref特征
    - `&i32.add(1)` -> `(*&i32).add(1)`
- 对于智能指针
    - `Box<i32>.add(1)` -> 触发Deref特征`Box<i32>.deref() -> &i32` 然后再自动解引用
- 对于&&&&v类型
    - `&&&&i32.add(1)` -> `&&&&i32.deref().deref().deref() -> &i32` 然后再自动解引用
```rust
// 标准库通过对引用泛型实现Deref特征解决嵌套的脱壳问题
impl<T: ?Sized> Deref for &T {
    type Target = T;
    fn deref(&self) -> &T {
        *self
    }
}
```
## 5. 三种Deref转换

1. 当`T: Deref<Target=U>`，可以将`&T`转换为`&U`
2. 当`T: DerefMut<Target=U>`，可以将`&mut T`转换为`&mut U`
3. 当`T: Deref<Target=U`，可以将`&mut T`转换为`&U`

*/

use std::ops::{Add, Deref};

/**
# `*`解引用
- 与`&`获取引用对应`*`运算符取出引用中的值
```rust
let x = 5;
let y = &x;

assert_eq!(5, x);
assert_eq!(5, *y);
```
*/
pub fn f02_01_star_mark() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/**
# 自动解引用行为

- 智能指针同样可用使用`*`运算符
```rust
let x = Box::new(1);
let sum = *x + 1;
```
- 智能指针和引用类型都存在自动解引用的行为
```rust
let x = Box::new(1);
let x = x.add(1);
println!("{x}");
```
- 本质上是Deref特征提供的自动类型转换功能
    - `x.add(1)`等价于`(*x).add(1);`
    - 编译器先检查`Box<i32>`类型发现没有add方法
    - 然后尝试触发Deref特征，转换为i32类型存在add方法，执行i32的add方法
- 触发这种行为需要函数传参或者是方法调用
    - 如果一个类型实现了Deref特征
    - 那么它的引用传递给方法或者函数的时候，会根据函数签名来决定是否进行隐式的类型转换
```rust
fn add_one(num: &i32) -> i32 {
    // + 运算符本身是add方法的语法糖
    // &i32实现了Deref特征
    // 等价转换关系
    // num + 1 -> num.add(1) -> (num.deref()).add(1)
    num + 1
}
let x = Box::new(1);
add_one(&x);
```
*/
pub fn f02_02_auto_deref() {
    let x = Box::new(1);
    let sum = *x + 1;
    println!("{sum}");

    let x = x.add(1);
    println!("{x}");

    let x = Box::new(1);
    let x = (*x).add(1);
    println!("{x}");

    fn add_one(num: &i32) -> i32 {
        num + 1
    }

    let x = Box::new(1);

    println!("{}", add_one(&x));
}

/**
# 手动实现Deref特征
- 自定义智能指针类型MyBox
    ```rust
    struct MyBox<T>(T);
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let i = MyBox(10);
    let j = i.add(1);
    println!("{j}");
    ```
*/
pub fn f02_03_deref_impl() {
    struct MyBox<T>(T);
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let i = &MyBox(10);
    let j = i.add(1);
    println!("{j}");
}
