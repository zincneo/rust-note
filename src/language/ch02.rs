/*!
# ch02 Rust基础概念

## 01 变量和可变性
1. [定义变量](./fn.f01_01_variable.html)
2. [变量可变性](./fn.f01_02_mutability.html)
3. [变量遮蔽](./fn.f01_03_shadowing.html)

## 02 基本类型

1. [标量类型](./fn.f02_01_scalar_type.html)
2. [组合类型](./fn.f02_02_compound_type.html)

## [03 语句和表达式](./fn.f03_statement_expression.html)

## [04 函数](./fn.f04_function.html)

## 05 流程控制
1. [if else](./fn.f05_01_if_else.html)
2. [for](./fn.f05_02_for.html)
3. [while和loop](./fn.f05_03_while_loop.html)
4. [continue和break](./fn.f05_04_continue_break.html)
5. [label标识符](./fn.f05_05_label.html)
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
    #[allow(unused)]
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

/**
# 语句和表达式
- Rust代码由语句和表达式组成
- 语句完成具体的操作但是没有返回值，以;结尾
  - 注意let关键字必须形成语句，没有返回值
- Rust中只要是返回值的都是表达式，表达式可以成为语句的一部分
  - 函数是表达式
  - {}是表达式
  - 表达式如果不返回值，就会隐式地返回一个空元组()

```rust
let _a = 10; // 变量定义语句
let x = 1;
// 有返回值的就是表达式
let _z = if x % 2 == 1 { "odd" } else { "even" };
```
*/
pub fn f03_statement_expression() {
    // 1. 语句
    {
        let _a = 10; // 变量定义语句

        // let a = (let a = 8); // let必须是语句，语句没有返回值，不能用作变量绑定
    }
    // 2. 表达式
    {
        let x = 1;
        // 有返回值的就是表达式
        let _z = if x % 2 == 1 { "odd" } else { "even" };
    }
}

/**
# 函数

## 函数要点
1. Rust使用fn关键字来定义函数
2. 函数名命名规则与变量相同
3. 函数位置可以随意放置，Rust不在乎在哪里定义，有定义即可
4. 函数定义形式`fn add(a: i32, b: i32) -> i32 {}`

## 参数列表
- Rust是强类型语言，必须为每一个参数标明类型

## 函数返回值
- 在参数列表后使用-> return_type注明返回值类型
- 当不标准返回值类型的时候等价与-> () 返回空元组
- 在{}函数体中可以使用return关键字来返回值也可以将最后一个表达式作为返回值，如果最后是一个语句且不是return语句则返回空元组

## 发散函数
- 返回值类型为!表示这个函数是一个发散函数
- 发散函数的含义是无法回到调用点继续执行代码
- 通常用于会导致死循环或者程序崩溃的函数
*/
pub fn f04_function() {
    #[allow(unused)]
    {
        fn dead_end() -> ! {
            panic!("你已经到了穷途末路，崩溃吧！");
        }

        fn forever() -> ! {
            loop {
                //...
            }
        }
    }
}

/**
# if else
- Rust提供if进行分支控制
- if和else if后需要跟一个返回布尔值的表达式，这个表达式不需要()包裹
- if代码块也是表达式，返回值是{}的最后一个表达式
- 当else if分支很多的情况下更推荐使用Rust提供的模式匹配
```rust
let n = 10;
let b = if n % 5 == 0 {
    1
}
else if n % 3 == 0 {
    2
} else {
    3
};
```
*/
pub fn f05_01_if_else() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

/**
# for
- Rust提供for in关键字作为迭代器的语法糖
- 迭代器见后续章节，这里看最简单的for用法
- `for i in 1..10`
  1. `start..end`语法可以生成一个连续的序列`[start,end)`
  2. for遍历这个序列，i表示每次遍历的元素
  3. Rust还提供了`start..=end`语法生成序列`[start,end]`
- `for i in &array`
  1. for循环迭代一个数组
  2. 通常迭代数组的引用，否则将发生元素所有权移动(见下一章所有权)，之后不能再使用数组
- 如果只需要迭代，不需要使用变量的时候可以使用_作为占位`for _ in ...`
*/
pub fn f05_02_for() {
    for i in 1..10 {
        println!("{}", i);
    }
    let arr = [10, 12, 13];
    for i in &arr {
        println!("{}", i);
    }
}

/**
# while和loop
- Rust提供while和loop两种循环方式
- while后需要一个返回布尔值的表达式，循环会执行到表达式返回false为止
- loop是一个简单的无限循环

```rust
let mut n = 0;
while n < 3 {
    n += 1;
    println!("{}", n);
}
loop {
    println!("死循环");
}
```
*/
pub fn f05_03_while_loop() {
    let mut n = 0;
    while n < 3 {
        n += 1;
        println!("{}", n);
    }
    /*
    loop {
        println!("死循环");
    }
    */
}

/**
# continue和break
- Rust提供continue和break两个关键字，必须作用于循环体内(for, while, loop代码块)
- continue跳过当前这一次的循环
- break跳出当前循环，并且break可以跟上一个值作为loop的返回值
```rust
    for i in 0..100 {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}");
    }

    for i in 0..1000_0000_000 {
        if i % 33 == 1 && i % 7 == 1 {
            break;
        }
    }

    let n = loop {
        break 3;
    };
```
*/
pub fn f05_04_continue_break() {
    for i in 0..100 {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}");
    }

    for i in 0..100_000_000 {
        if i % 33 == 1 && i % 7 == 1 {
            break;
        }
    }

    let _n = loop {
        break 3;
    };
}

/**
# label标识符
- Rust提供label标识符用作为循环体的标记(for, while, loop)
- label的语法('label_name: for/while/loop)
- 结合continue和break关键字使用`continue/break 'label_name`，在嵌套循环体的时候明确指定作用于哪个循环体

```rust
    let n = 'out: loop {
        'inner: loop {
            break 'out 10;
        }
    };
```
*/
#[allow(unused)]
pub fn f05_05_label() {
    let n = 'out: loop {
        'inner: loop {
            break 'out 10;
        }
    };
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

    #[test]
    fn ch02_03() {
        assert_eq!(f03_statement_expression(), ());
    }

    #[test]
    fn ch02_04() {
        assert_eq!(f04_function(), ());
    }

    #[test]
    fn ch02_05() {
        assert_eq!(f05_01_if_else(), ());
        assert_eq!(f05_02_for(), ());
        assert_eq!(f05_03_while_loop(), ());
        assert_eq!(f05_04_continue_break(), ());
        assert_eq!(f05_05_label(), ());
    }
}
