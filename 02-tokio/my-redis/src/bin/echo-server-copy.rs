/*
# 回声服务
- 实现 web 服务器，往往会选择实现一个回声服务
- 该服务会将用户的输入内容直接返回给用户，就像回声壁一样
*/
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        // 每条连接创建一个异步任务进行处理，不会阻塞当前任务
        tokio::spawn(async move {
            // io::split 可以用于任何同时实现了 AsyncRead 和 AsyncWrite 的值，它的内部使用了 Arc 和 Mutex 来实现相应的功能
            // 觉得这种实现有些重，可以使用 Tokio 提供的 TcpStream，它提供了两种方式进行分离 TcpStream::split和TcpStream::into_split
            let (mut rd, mut wr) = socket.split();

            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}
