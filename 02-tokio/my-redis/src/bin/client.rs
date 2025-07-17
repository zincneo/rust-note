use bytes::Bytes;
use mini_redis::client;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = tokio::sync::oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // 创建新的缓存通道，缓冲队列的长度是32
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    let tx2 = tx.clone();
    // 管理客户端连接服务端的任务
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收从客户端其他的任务中发送过来的信息
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    // 从服务端获取到res
                    let res = client.get(&key).await;
                    let _ = resp.send(res); // 再通过oneshot发送到原来的任务里面
                }
                Set { key, value, resp } => {
                    let res = client.set(&key, value).await;
                    let _ = resp.send(res);
                }
            }
        }
    });
    // 有两个任务，我们希望异步并发，所有有两个tx
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }
        let res = resp_rx.await;
        println!("GOT (Get) = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            resp: resp_tx,
        };

        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }
        let res = resp_rx.await;
        println!("Got (Set) = {:?}", res);
    });

    // 然后通过await让任务按照想要的顺序进行执行
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
