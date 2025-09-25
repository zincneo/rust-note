# 消息通道

标准库提供了通道std::sync::mpsc，其中mpsc是multiple producer, single consumer的缩写，代表了该通道支持多个发送者，但是只支持唯一的接收者。 当然，支持多个发送者也意味着支持单个发送者

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者) (Sender, Reciver)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
}
```

- `resv`方法会阻塞当前线程直到拿到数据或者发送者被Drop
- `try_resv`方法则不会阻塞线程
- `Reciver`类型本身实现了迭代器特征，因此可以使用for直接循环接收

## 同步和异步通道

- `mpsc`的`channel`方法获得的是异步的通道，也就是无论接收者是否接收了消息都不会阻塞发送者发送消息
- `mpsc`的`sync_channel`方法获得的是同步的通道，当通道被占满时继续发送会阻塞发送者，直到接收者取数据使得通道有剩余容量，该方法的参数用于指定通道的容量

```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx)= mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}
```
