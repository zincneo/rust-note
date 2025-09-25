# Rust线程

不同语言对于线程的实现差异很大，总的来说有以下三种:

1. 由于操作系统提供了创建线程的 API，因此部分语言会直接调用该 API 来创建线程，因此最终程序内的线程数和该程序占用的操作系统线程数相等，一般称之为1:1 线程模型，例如 Rust
2. 还有些语言在内部实现了自己的线程模型（绿色线程、协程），程序内部的 M 个线程最后会以某种映射方式使用 N 个操作系统线程去运行，因此称之为M:N 线程模型，其中 M 和 N 并没有特定的彼此限制关系。一个典型的代表就是 Go 语言
3. 还有些语言使用了 Actor 模型，基于消息传递进行并发，例如 Erlang 语言

Rust线程的基础API(在std::thread模块下)

1. `spawn` 创建一个线程，参数是一个无参的闭包，该线程执行这个闭包的代码
2. `sleep` 在线程内部使用，参数是一个`Duration`，使得该线程休眠指定的时间
3. `join` 是`spawn`方法返回值的方法，用于在父线程提供阻塞等待子线程执行结束

## Rust线程终止

- 主函数main线程一旦结束会强制终止子线程
- 非main线程创建的子线程则会运行到自身结束为止，父线程结束不会强制其结束运行(但同样，main线程结束也会强制终止这些子孙线程)

```rs
use std::thread;
use std::time::Duration;
fn main() {
    // 创建一个线程A
    let new_thread = thread::spawn(move || {
        // 再创建一个线程B
        thread::spawn(move || {
            loop {
                println!("I am a new thread.");
            }
        })
    });

    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
    println!("Child thread is finish!");

    // 睡眠一段时间，看子线程创建的子线程是否还在运行
    thread::sleep(Duration::from_millis(100));
}
  
```
