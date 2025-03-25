/*!
# ch02 Rust基础概念

## 01 变量和可变性
1. [定义变量](./fn.f01_01_variable.html)
2. [变量可变性](./fn.f01_02_mutability.html)
3. [变量遮蔽](./fn.f01_03_shadowing.html)

*/

/**
# 定义变量

- Rust中定义变量的关键字是let
```rust
let variable;
```

- Rust是强类型语言，每个变量的类型唯一且不可以改变，变量类型在变量名后指明`var_name: type`
```rust
let variable: i32;
```

- Rust的编译器很强大，如果不手动指明类型，编译器会根据首次绑定值进行类型推导，对于推导不出来的编译将无法通过
```rust
let variable;
variable = 10;
```

- 编译器会检查变量是否使用，如果存在没有使用的变量，编译器将会警告，对于使用_开头的变量不会警告，或者使用宏#[allow(unused_variables)]
- 变量名命名规则
  1. 英文字符、下划线、数字0~9
  2. 变量名遵循蛇形命名法(snake_case)，不遵循的编译器会警告

- Rust中定义常量的关键字是const
  1. 与变量对应，命名遵循大写蛇形命名法(SNAKE_CASE)
  2. 常量必须手动标注类型，未标注类型编译器会报错

```rust
const I32_VAR: i32 = 20;
```
*/
pub fn f01_01_variable() {
    #[allow(unused_variables)]
    {
        let variable = 10;
        const I32_VAR: i32 = 20;
    }
}

/**
# 变量可变性
- Rust中变量默认不可变
  - 首次绑定值之后不能再次使用=运算符绑定新的值
- Rust中使用可变的变量需要在let关键字之后加上mut关键字
```rust
let a; // 不可变
a = 10;
let mut b; // 可变
b = 10;
b = 20;
```
*/
pub fn f01_02_mutability() {
    let mut variable = 10;
    variable += 10;
    println!("{variable}");
}

/**
# 变量遮蔽
- Rust中提供了变量遮蔽(shadowing)的能力
- 在同一作用域中可以定义同名的变量
- 在嵌套的子作用域中也可以定义和上级作用域同名的变量
- 后定义的同名变量会遮蔽前定义的同名变量
- 当离开嵌套的子作用域之后，上级作用域的同名变量再次可以使用
```rust
let a = 10;
let a = 20; // 遮蔽上一个
{
    let a = 30; // 遮蔽外部的a
    println!("{a}"); // 30
}
println!("{a}"); // 20
```
*/
#[allow(unused_variables)]
pub fn f01_03_shadowing() {
    let a = 10;
    let a = 20; // 遮蔽上一个
    {
        let a = 30; // 遮蔽外部的a
        println!("{a}"); // 30
    }
    println!("{a}"); // 20
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch02_01() {
        assert_eq!(f01_01_variable(), ());
        assert_eq!(f01_02_mutability(), ());
        assert_eq!(f01_03_shadowing(), ());
    }
}
