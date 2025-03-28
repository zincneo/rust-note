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
0xffff_00ff_i128; // 十六进制
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
    0xffff_00ff_i128; // 十六进制
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
- 元组可以使用.索引来访问内容
- 在定义变量的时候可以使用`(name1, name2, ...) = tuple`来解构一个元组
```rust
(32, 0.0, 1); // 一个(i32, f64, i32)类型元组的字面值
(); // 空元组()类型的字面值
let tup = (0, 1.1, 2);
tup.0; // 使用.索引访问内容
let (_x, _y, _z) = tup;
```

## 数组
- Rust提供的数组使用`[element_type;length]`来表示
- 长度是一个usize类型的值
- 初始化的使用可以使用`[element_value;length]`将整个数组所有元素初始化为相同的值
- Rust中使用下标越界访问数组将会导致程序崩溃

```rust
[100;10]; // 一个类型为[i32;10]的数组，所有元素值都是100
```
*/
pub fn f02_02_compound_type() {
    let _i = (32, 0.0, 1);
    let _i = [100; 10];
    let tup = (0, 1.1, 2);
    tup.0; // 使用.索引访问内容
    let (_x, _y, _z) = tup;
}
