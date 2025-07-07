#![allow(dead_code)]
#![allow(unused)]

/**
# 线程模型
- Rust标准库`std::thread`提供和os线程一比一的线程模型
- 常用方法
    - spawn创建一个新的线程
        - 参数是一个闭包，线程执行传入的闭包
        - 不会阻塞当前的线程
        - 返回一个JoinHandle类型的变量
        - 返回值执行join方法可以阻塞当前线程直到子线程运行结束
    - sleep方法，让当前线程休眠指定时间
- Rust中main线程结束的时候就结束所有线程
- 父线程非main线程的线程在父线程结束的时候不会结束
*/
pub fn f01_thread() {
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    let t1 = spawn(|| {
        for i in 0..3 {
            sleep(Duration::from_millis(1000));
            println!("first thread: {i}");
        }
    });

    let t2 = spawn(|| {
        for i in 0..3 {
            sleep(Duration::from_millis(1000));
            println!("second thread: {i}");
        }
    });

    t1.join();
    t2.join();
}

/**
# 线程屏障
- 线程安全的引用计数std::sync::Arc
- std::sync::Barrier提供让多个线程执行到某个点之后一起执行的能力
- Barrier::new传入线程的数量，当调用Barrier::wait方法的线程数量达到new中传入的数量之后继续执行
*/
pub fn f02_barrier() {
    use std::sync::{Arc, Barrier};
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for i in 0..6 {
        let b = barrier.clone();
        handles.push(std::thread::spawn(move || {
            println!("before {i} wait");
            b.wait();
            println!("after {i} wait, strong count {}", Arc::strong_count(&b));
        }));
    }

    for handle in handles {
        handle.join();
    }
}

static mut VAL: usize = 0;
static INIT: std::sync::Once = std::sync::Once::new();
/**
# 只调度一次的函数

- 一些场景需要某个函数在多线程环境下只能被调度一次
- 标准库提供了`std::sync::Once`来实现这一功能
- 该类型的实例多次调用call_once方法(参数是一个闭包), 只有第一次会执行
*/
pub fn f03_call_once() {
    use std::thread;
    // 100个线程，每一个都执行call_once，只会执行一次
    (0..100)
        .map(|idx| {
            thread::spawn(move || {
                thread::sleep(std::time::Duration::from_millis(200));
                INIT.call_once(|| unsafe {
                    VAL = idx;
                });
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| {
            handle.join();
        });
    println!("{:?}", unsafe { VAL });
}
