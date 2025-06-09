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

1. [泛型参数](./ch07_01_generics/fn.f01_generics.html)
2. [const泛型](./ch07_01_generics/fn.f02_const_generics.html)

## 02 特征

以文件系统举例，文件操作主要包含四个：open 、write、read、close，这些操作可以发生在硬盘，可以发生在内存，还可以发生在网络 IO，如果要为每一种情况单独实现一套代码就过于复杂了。将这些方法抽象出来，在Rust中就要使用特征trait这一概念。在其他的编程语言当中可能会通过继承接口类的方式实现(java，c++)，本质上都是多态的表现形式，即不同的类型具有相同的行为。

1. [特征定义和实现](./ch07_02_trait/fn.f01_trait.html)
2. [使用特征](./ch07_02_trait/fn.f02_use_trait.html)

## 03 深入特征

1. [关联类型](./ch07_03_advance_trait/fn.f01_type.html)
2. [调用同名方法](./ch07_03_advance_trait/fn.f02_same_name.html)
3. [默认泛型参数](./ch07_03_advance_trait/fn.f03_default_generic.html)
4. 特征约束特征
    - 和类型约束特征语法相同`Trait: Trait1 + Trait2`
    - 在实现了约束特征之后才能实现该特征
*/

pub mod ch07_01_generics;
pub mod ch07_02_trait;
pub mod ch07_03_advance_trait;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch07_01() {
        assert_eq!(ch07_01_generics::f01_generics(), ());
        assert_eq!(ch07_01_generics::f02_const_generics(), ());
    }

    #[test]
    fn ch07_02() {
        assert_eq!(ch07_02_trait::f01_trait(), ());
        assert_eq!(ch07_02_trait::f02_use_trait(), ());
    }

    #[test]
    fn ch07_03() {
        assert_eq!(ch07_03_advance_trait::f01_type(), ());
        assert_eq!(ch07_03_advance_trait::f02_same_name(), ());
        assert_eq!(ch07_03_advance_trait::f03_default_generic(), ());
    }
}
