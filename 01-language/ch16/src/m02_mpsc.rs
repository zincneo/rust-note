#![allow(dead_code)]
#![allow(unused)]

/**
# 线程间通信:消息通道
- 标准库提供了通道std::sync::mpsc
- multiple producer, single consumer 多发送者，单接收者
- 异步通道:`std::sync::mpsc`默认异步
    - 不论接收者线程是否在接收，都不会导致发送者线程阻塞
    - mpsc::channel返回一个元组，元组.0为发送者，元组.1为接收者
    - 发送者可以使用send方法发送信息(不会导致当前线程阻塞)
    - 接收者可以使用recv方法接收信息(阻塞当前线程)
    - 传输数据也遵循所有权规则
        - 实现Copy特征的值会传输一份复制
        - 没有实现Copy特征的值会将所有权传递过去
        - 接收者实现了IntoIterator特征，迭代器会阻塞到发送者被drop为止，因此可以使用for语法糖直接迭代接收者
- 当所有**发送者**或者所有**接收者**离开各自作用域被drop之后，通道关闭
- 在编译期就确定何时会关闭
*/
pub fn f01_async_mpsc() {
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    let t1 = spawn(move || {
        for _ in 0..5 {
            sleep(Duration::from_secs(100));
            tx.send(5); // 不会阻塞当前线程
        }
    });

    let t2 = spawn(move || {
        for _ in 0..5 {
            println!("{:?}", rx.recv()); // 会阻塞当前线程
        }
    });
}

/**
# try_recv

- 相比于recv，try_recv不会阻塞当前线程
*/
pub fn f02_try_recv() {
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    let t1 = spawn(move || {
        for _ in 0..5 {
            sleep(Duration::from_secs(100));
            tx.send(5); // 不会阻塞当前线程
        }
    });

    let t2 = spawn(move || {
        for _ in 0..5 {
            println!("{:?}", rx.try_recv()); // 不会阻塞当前线程
        }
    });
}

/**
# 同步通道
- std::sync::mpsc::sync_channel
- 当通道已经发送消息填满缓存的时候继续往通道发送消息会阻塞当前所在线程，直到通道缓存不满
- 默认缓存大小0，函数的参数是缓存的大小
*/
pub fn f03_sync_channel() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::sync_channel(1);

    let handle = thread::spawn(move || {
        println!("发送1之前");
        tx.send(1).unwrap();
        println!("发送1之后");
        println!("发送2之前");
        tx.send(2).unwrap();
        println!("发送2之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}
