#[tokio::main]
async fn main() {
    // tokio提供的异步通道
    //
    // 1. 多发送者，单接收者
    {
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);

        for i in 0..40 {
            let tx = tx.clone();
            tokio::spawn(async move {
                let _ = tx.send(i).await;
            });
        }
        // 注意这种情况下要手动提前释放掉tx，否则会导致死锁
        // rx在等待所有的tx释放，但是上面由于有一个tx没有被移动到其他异步任务中，所以在当前作用域不结束就不会Drop
        std::mem::drop(tx);
        let mut count = 0;
        while let Some(val) = rx.recv().await {
            count += 1;
            println!("multiple - single {count}: {val}");
        }
    }

    // 2. 单发送者，单接收者(一次只能发送一条消息)
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let _ = tx.send(1); // 只能发送一次消息
        });
        if let Ok(val) = rx.await {
            println!("single - single {val}");
        }
    }

    // 3. 多发送者，多接收者(每次发送的消息会广播到所有接收者中)
    {
        let (tx, rx) = tokio::sync::broadcast::channel(10);
        let mut handles = vec![];
        for i in 0..3 {
            let mut rx = rx.resubscribe();
            let handle = tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(i * 100)).await;
                while let Ok(val) = rx.recv().await {
                    println!("get {val} in job {i}");
                }
            });
            handles.push(handle);
        }
        for i in 0..10 {
            let tx = tx.clone();
            tokio::spawn(async move {
                let _ = tx.send(i);
            });
        }
        // 手动关闭tx，否则下面await其他接收者的时候又造成等待tx关闭了，死锁
        std::mem::drop(tx);
        for handle in handles {
            let _ = handle.await;
        }
    }

    // 4. 单发送者，多接收者(只保存一条最新的消息，因此接收者只能看到最近的一条消息)
    {
        // 创建通道的时候必须就有初始消息
        let (tx, mut rx) = tokio::sync::watch::channel(1);
        // 每次发送会覆盖通道中的消息，接收的时候只会接收到最新的消息
        println!("{:?}", rx.borrow_and_update());
        let _ = tx.send(10);
        println!("{:?}", rx.borrow_and_update());
    }
}
