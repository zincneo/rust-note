/*!
# 线程间通信:消息通道

- 标准库提供了通道std::sync::mpsc
- multiple producer, single consumer 多发送者，单接收者

## [1. 异步通道](./fn.f02_01_use.html)
## [2. 不阻塞的接收方法](./fn.f02_02_try_revc.html)
## [3. 同步通道](./fn.f02_03_sync.html)
## 4. 关闭通道
- 当所有**发送者**或者所有**接收者**离开各自作用域被drop之后，通道关闭
- 在编译期就确定何时会关闭
## 多发送者多接收者
- 可以使用第三方的并发工具库Crossbeam中提供的channel
*/

/**
# 异步通道
- std::sync::mpsc默认是异步的，不论接收者线程是否在接收，都不会导致发送者线程阻塞
- mpsc::channel返回一个元组，元组.0为发送者，元组.1为接收者
- 发送者可以使用send方法发送信息
- 接收者可以使用recv方法接收信息(阻塞当前线程)
- 传输具有所有权的数据
    - 实现Copy特征的会直接复制一份
    - 没有Copy特征则发生所有权转移
- 接收者实现了IntoIterator特征，迭代器会阻塞到发送者被drop为止，因此可以使用for语法糖直接迭代接收者
    ```rust
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4, 5];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    if let Ok(val) = val {
        println!("Got {}", val);
    }
    for received in rx {
        println!("Got: {}", received);
    }
    ```
*/
pub fn f02_01_use() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4, 5];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    let val = rx.recv();
    if let Ok(val) = val {
        println!("Got: {}", val);
    }
    for received in rx {
        println!("Got: {}", received);
    }
}

/**
# 不阻塞的try_revc方法

- 如果不希望接收方法阻塞当前线程，可以使用try_revc方法
    ```rust
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![3, 2, 1];
        thread::sleep(Duration::from_millis(500));
        for val in vals {
            tx.send(val).unwrap();
        }
    });

    if let Ok(val) = rx.try_recv() {
        println!("{val}");
    } else {
        println!("没接收到，不阻塞线程");
    }
    ```
*/
pub fn f02_02_try_revc() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![3, 2, 1];
        thread::sleep(Duration::from_millis(500));
        for val in vals {
            tx.send(val).unwrap();
        }
    });

    if let Ok(val) = rx.try_recv() {
        println!("{val}");
    } else {
        println!("没接收到，不阻塞线程");
    }
}

/**
# 同步通道

1. std::sync::mpsc::sync_channel返回一个同步的通道
2. 和异步通道的区别在于发送者发送数据后线程会阻塞到接收者接收数据为止
3. sync_channel的参数是通道的缓存数量，只有缓存不足才会阻塞发送者
    ```rust
    let (tx, rx) = mpsc::sync_channel(1);

    let handle = thread::spawn(move || {
        println!("发送1之前");
        tx.send(1).unwrap();
        println!("发送1之后");
        println!("发送2之前");
        tx.send(2).unwrap();
        // 缓存队不足，如果不接收1，该线程一直阻塞
        println!("发送2之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
    ```
*/
pub fn f02_03_sync() {
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
