/*!
# ch02 Rust基础概念

## 01 变量和可变性
1. [定义变量](./fn.f01_01_variable.html)
2. [变量可变性](./fn.f01_02_mutability.html)
3. [变量遮蔽](./fn.f01_03_shadowing.html)

## 02 基本类型

1. [标量类型](./fn.f02_01_scalar_type.html)
2. [组合类型](./fn.f02_02_compound_type.html)

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

/**
# 标量类型
- 标量类型(Scalar Type)表示一个单一的值，Rust提供四种原始(primitive)的标量类型
  1. Integer Types
  2. Floating-point Types
  3. Boolean Type
  4. Character Type

## 整数类型
- Rust提供多种整数类型，类型明确指明有无符合和占内存多少个bit位
  1. i8 u8
  2. i16 u16
  3. i32 u32
  4. i64 u64
  5. i128 u128
  6. isize usize
- 整数字面值会编译器会默认推导为i32
- 整数字面值可以在最后手动写明类型
- 整数字面值可以使用_进行分割提高可读性
- 整数字面值支持以下进制
  - 十进制 无特殊
  - 十六进制 字面值开头加上0x
  - 八进制 字面值开头加上0o
  - 二进制 字面值开头加上0b
  - u8支持的字符串字面值 字面值使用b''包裹一个ASCII码字符


```rust
98_222_000; // 十进制
0xffff_00ff; // 十六进制
0o777_707; // 八进制
0b00111100; // 二进制
100i128; // i128的整数字面值
b'A'; // 数字字符字面值
let _i = 100; // 默认推导i32
let _i: i128 = 100; // 手动指明整数类型
```

## 浮点数类型

- Rust提供两种精度类型的浮点数
  1. f32
  2. f64
- 字面值默认推导为f64
- 浮点数支持科学计数法字面值e,E均可

```rust
let _f = 10.0; // 默认推导f64
let _f: f32 = 10.0; // 手动指明浮点数类型
let _f = 2.5e-3; // 科学计数法
let _f = 1E12;
```

## 数学运算符

- Rust提供+,-,*,/,%这些基本的数学运算符
- 数值类型在进行运算的时候必须类型一致，不同的整数类型之间无法直接运算
- +,-做为单目运算符时表示数值正负

```rust
// addition
let sum = 5 + 10;
// subtraction
let difference = 95.5 - 4.3;
// multiplication
let product = 4 * 30;
// division
let quotient = 56.7 / 32.2;
let truncated = -5 / 3; // Results in -1
// remainder
let remainder = 43 % 5;
```

## 布尔值

- Rust提供bool类型
- 字面值为`true`和`false`两个
- 注意不能当作c那样强制转换为数值类型

## 字符

- Rust提供的字符类型char为定长为4byte(32bit)的Unicode编码字符
- 使用单引号''包裹

```rust
let c = 'z';
let z: char = 'ℤ'; // 手动指明类型
let heart_eyed_cat = '😻';
```
*/
pub fn f02_01_scalar_type() {
    98_222_000; // 十进制
    0xffff_00ff; // 十六进制
    0o777_707; // 八进制
    0b00111100; // 二进制
    100i128; // i128的整数字面值
    b'A'; // 数字字符字面值
    let _i = 100; // 默认推导i32
    let _i: i128 = 100; // 手动指明整数类型

    let _f = 10.0; // 默认推导f64
    let _f: f32 = 10.0; // 手动指明浮点数类型
    let _f = 2.5e-3; // 科学计数法
    let _f = 1E12;

    let _c = 'z';
    let _z: char = 'ℤ'; // 手动指明char类型
    let _heart_eyed_cat = '😻';
}

/**
# 组合类型
- Rust提供两种原始(primitive)的组合类型
  1. 元组tuple
  2. 数组

## 元组
- Rust提供的元组类型用()来包裹一组值
- ()中通过逗号分割包裹指定数量的值(value1, value2, value3, ...)
- 编译器根据包裹的每一个值推导出确定的一个元组类型(type1, type2, type3, ...)
- 元组包裹的内容可以为空，这个类型被视为一种特殊的类型空元组
  - 类型和值一模一样都是()
  - 这个类型在函数和语句中会再次提到具体的用途
```rust
(32, 0.0, 1); // 一个(i32, f64, i32)类型元组的字面值
(); // 空元组()类型的字面值
```

## 数组
- Rust提供的数组使用[element_type;length]来表示
- 长度是一个usize类型的值
- 初始化的使用可以使用[element_value;length]将整个数组所有元素初始化为相同的值

```rust
[100;10]; // 一个类型为[i32;10]的数组，所有元素值都是100
```
*/
pub fn f02_02_compound_type() {
    let _i = (32, 0.0, 1);
    let _i = [100; 10];
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

    #[test]
    fn ch02_02() {
        assert_eq!(f02_01_scalar_type(), ());
        assert_eq!(f02_02_compound_type(), ());
    }
}
