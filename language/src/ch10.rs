/*!
# Rust错误处理

Rust中错误主要分为两类:
1. **可恢复的错误**:通常用于从系统全局角度来看可以接受的错误，例如处理用户的访问、操作等错误，这些错误只会影响某个用户自身的操作进程，而不会对系统的全局稳定性产生影响
2. **不可恢复的错误**:该错误通常是全局性或者系统性的错误，例如数组越界访问，系统启动时发生了影响启动流程的错误等等，这些错误的影响往往对于系统来说是致命的

对于很多编程语言，并不会区分以上两种错误，而是直接采用异常机制处理，但Rust没有提供异常机制，而是提供了对应的两种处理方式
1. `Result<T, E>`枚举用于处理可恢复的错误
2. `panic!`用于处理不可恢复的错误

## [不可恢复的错误](./fn.f01_panic.html)

## [可恢复的错误](./fn.f02_result.html)

## [错误传播](./fn.f03_question_mark.html)

## [组合器](./fn.f04_combine.html)
*/

/**
# 不可以恢复的错误

- 对于严重到影响系统运行的错误，触发程序崩溃(panic)是很好的解决方式
    - 举例来说，系统启动阶段读取必要的文件失败，系统无法启动，这属于不可恢复的错误，直接触发panic
- 被动触发

```rust
let v = vec![1, 2, 3];
v[99]; // 数组越界访问，被动触发panic
```

- 主动调用`panic`宏

```rust
fn main() {
    panic!("crash and burn");
}

- 崩溃的时候打开函数调用栈的命令
    `RUST_BACKTRACE=1 cargo run`

- 可恢复的错误Result枚举提供unwrap方法，该方法在值为Ok的时候会返回包裹的值，值为Err的时候不处理而是触发panic

- 主线程`panic`才会导致整个程序崩溃，子线程触发`panic`只会终止那个线程

- 使用`panic`的场景
    - 示例、原型、测试
    - 确切知道程序正确的情况下，使用会触发panic的unwrap方法比处理Result简单
    - 可能导致全局有害状态的时候
        1. 非预期的错误
        2. 后续代码的运行会受到显著影响
        3. 内存安全问题
```
*/
pub fn f01_panic() {}

/**
# 可恢复的错误
- Result枚举用来处理可恢复的错误
- Result::Ok值用来包裹程序运行正确的情况下的值，Result::Err值用来包裹程序运行失败情况下的信息
- 通过模式匹配match来处理成功和失败的情况
- Result枚举本身包含在标注库std::result中
    - 该枚举在prelude中包含，因此可以直接使用
*/
#[allow(unused)]
pub fn f02_result() {
    use std::fs::File;

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
}

/**
# 错误传播
- 当一个函数的返回值为Result类型的函数体内，可以使用`?`向上级传播错误

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

- 可以认为就是match的语法糖

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    let mut s = match f.read_to_string(&mut s) {
        Ok(s) => s,
        Err(err) => return Err(e);
    }
    Ok(s)
}
```

- 因此`?`的作用就是当Result为Ok值的时候直接解构出包裹的值，当Result为Err的时候结束函数，向上传播错误
- 对于Option枚举使用?则Option为Some值的时候解构出包裹的值，当Option为None的时候结束函数，返回None
    - 下面的代码示例展示了如果将`?`运算符和链式调用结合起来使用

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

- Rust的主函数还支持返回值为`Result<(), Box<dyn std::error::Error>>`类型
*/
pub fn f03_question_mark() {}

/**
# 组合器

由Option和Result类型提供的一系列方法，实现Option，Result值或类型的转换

- or和and
    - 参数是调用者同类型的值
    - ok表达式按照顺序求值，若任何一个表达式的结果是 Some 或 Ok，则该值会立刻返回
    - and若两个表达式的结果都是 Some 或 Ok，则第二个表达式中的值被返回。若任何一个的结果是 None 或 Err ，则立刻返回
    ```rust
    let s1 = Some("some1");
    let s2 = Some("some2");
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
    assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
    ```
- or_else和and_then
    - 和or以及and的差别只在于参数是一个返回同类型值的闭包
    ```rust
    // or_else with Option
    let s1 = Some("some1");
    let fn_some = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };
    assert_eq!(s1.or_else(fn_some), s1); // Some1 or_else Some2 = Some1
    // or_else with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };
    assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
    ```
- filter方法
    - 就是迭代器上的filter方法，Option类型实现了Iterator特征
    ```rust
    let s1 = Some(3);
    assert_eq!(s1.filter(|x| x % 2 == 0), None);
    ```
- map和map_err
    - map可以将Some或者Ok的值映射为另外一个
    - 要让Err中包裹的值可以被映射，就需要使用map_err
    ```rust
    let s1 = Some("abcde");
    let s2 = Some(5);
    assert_eq!(s1.map(|s: &str| s.chars().count()), s2); // Some1 map = Some2

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() };

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");
    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2
    ```
- map_or和map_or_else
    - map_or在map基础上提供一个基础值，处理None的时候会返回基础值
    - map_or_else区别与map_or的地方是传入的是一个返回基础值的闭包
- ok_or和ok_or_else
    - 这两个可以将Option转换为Result
*/
pub fn f04_combine() {
    let s1 = Some("abcde");
    let s2 = Some(5);
    assert_eq!(s1.map(|s: &str| s.chars().count()), s2); // Some1 map = Some2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch10_01() {
        assert_eq!(f01_panic(), ());
    }

    #[test]
    fn ch10_02() {
        assert_eq!(f02_result(), ());
    }

    #[test]
    fn ch10_03() {
        assert_eq!(f03_question_mark(), ());
    }

    #[test]
    fn ch10_04() {
        assert_eq!(f04_combine(), ());
    }
}
