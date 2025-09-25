pub fn run() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    // 异步通道演示
    {
        let (tx, rx) = mpsc::channel();

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

    // 同步通道演示
    {
        let (tx, rx) = mpsc::sync_channel(0);

        let handle = thread::spawn(move || {
            println!("发送之前");
            tx.send(1).unwrap();
            println!("发送之后");
            tx.send(2).unwrap(); // 线程会一直阻塞到通道清空才会继续执行
        });

        println!("睡眠之前");
        thread::sleep(Duration::from_secs(3));
        println!("睡眠之后");

        println!("receive {}", rx.recv().unwrap());
        println!("拿走通道所有数据才能让子线程结束运行");
        println!("receive {}", rx.recv().unwrap());
        handle.join().unwrap();
    }
}
