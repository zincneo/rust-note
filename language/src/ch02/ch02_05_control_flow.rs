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
