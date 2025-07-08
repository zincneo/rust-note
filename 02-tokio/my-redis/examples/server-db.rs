use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use std::collections::HashMap;
// tokio的Mutex只应该在生命周期在多个.await之间持有的时候才应该使用
// 锁竞争不多的情况下要使用标准库的互斥锁
// 锁竞争很多可以考虑性能更好的第三方库，例如parking_lot::Mutex
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:6397").await.unwrap();
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = tcp_listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.lock()
                    .unwrap()
                    .insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            } // lock释放
            Command::Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            } // lock释放
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
