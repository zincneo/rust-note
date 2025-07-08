use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // 其实一次只能接受和处理一条 TCP 连接，只有等当前的处理完并结束后，才能开始接收下一条连接
        // 原因在于 loop 循环中的 await 会导致当前任务进入阻塞等待，也就是 loop 循环会被阻塞
        // process(socket).await;
        // 让这个任务真正并发
        // 每一条进来的连接都生成( spawn )一个新的任务, 然后在该任务中处理连接
        // `socket` 的所有权将被移动到新的任务中，并在那里进行处理
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    // 使用哈希表在每次客户端的请求过来的时候存储可能需要存储的数据
    use std::collections::HashMap;

    // 当前这个db被process的一个任务独享，其他连接在主函数中调度的时候无法获取当同一的db
    let mut db = HashMap::new();

    // `mini-redis` 提供的便利函数，使用返回的 `connection` 可以用于从 socket 中读取数据并解析为数据帧
    let mut connection = Connection::new(socket);

    // 使用 `read_frame` 方法从连接获取一个数据帧：一条redis命令 + 相应的数据
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // key String -> value Vec<u8>
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        // 返回响应给客户端
        connection.write_frame(&response).await.unwrap();
    }
}
