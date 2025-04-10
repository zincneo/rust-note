/*!
# Rust并发编程

## 0. 并发和并行

- 并发(Concurrent): 操作系统控制多个任务轮换执行，对于用户切换的够快就好像同时执行
- 并行(Parallel): 真的有多个处理器同时在处理多个任务
- 实际情况是在现代操作系统中并发并行通常同时存在
- Rust提供的并发编程是基于操作系统线程的1:1模型
- Rust也提供了异步的M:N模型，比如第三方异步库tokio
- 并发编程容易出现的风险
    1. 竞态条件(race conditions)，多个线程以非一致性的顺序访问资源
    2. 死锁(deadlocks)，两个线程都想使用某个资源，但是又都在等待对方释放资源后才能使用，结果最终都无法继续执行
    3. 一些因为多线程导致的很隐晦的 BUG，难以复现和解决

## [1. 使用多线程](./fn.f01_use_thread.html)
## [2. 线程间通信:消息通道](./ch17_02_mpsc/index.html)
*/

/**
# 使用多线程

- std::thread::spawn用于创建线程
    - 该函数传入一个闭包，闭包将在新线程内执行
    - 该函数返回一个JoinHanle类型的值，该类型提供一个join方法，将阻塞当前线程直到子线程执行结束
    - 如果要将值传递到子线程中需要在闭包上加move关键字强调以所有权转移的方式捕获变量
    ```rust
    use std::thread;
    use std::time::Duration;

    let handle_1 = thread::spawn(|| {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(1000));
            println!("first thread: {i}");
        }
    });

    let handle_2 = thread::spawn(|| {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(1000));
            println!("second thread: {i}");
        }
    });

    for i in 0..3 {
        thread::sleep(Duration::from_millis(500));
        println!("main thread: {i}");
    }

    handle_1.join().unwrap();
    handle_2.join().unwrap();
    ```
- Rust中main线程结束的时候就结束所有线程
- 父线程非main线程的线程在父线程结束的时候不会结束
*/
pub fn f01_use_thread() {
    use std::thread;
    use std::time::Duration;

    let handle_1 = thread::spawn(|| {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(1000));
            println!("first thread: {i}");
        }
    });

    let handle_2 = thread::spawn(|| {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(1000));
            println!("second thread: {i}");
        }
    });

    for i in 0..3 {
        thread::sleep(Duration::from_millis(500));
        println!("main thread: {i}");
    }

    handle_1.join().unwrap();
    handle_2.join().unwrap();
}

pub mod ch17_02_mpsc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch17_01() {
        assert_eq!(f01_use_thread(), ());
    }

    #[test]
    fn ch17_02() {
        assert_eq!(ch17_02_mpsc::f02_01_use(), ());
    }
}
