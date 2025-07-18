use std::io::{self, Read, Write};
fn main() -> io::Result<()> {
    // 标准库的net模块提供了Tcp相关的类用来建立和发送TCP连接
    use std::net::TcpListener;
    // 创建一个Tcp服务器，绑定到21001端口
    let listener = TcpListener::bind("127.0.0.1:21001")?;
    // 同步代码，该迭代器会阻塞到进来一个连接才结束，然后又进入阻塞，因此就可以一条一条连接处理
    for stream in listener.incoming() {
        // 连接成功会返回一个TcpStream类，该类型实现了io模块下的读写特征
        let stream = stream?;
        handle_stream(stream)?;
        println!("----当前连接处理完毕，等待下一个连接----")
    }
    Ok(())
}

fn handle_stream(mut stream: std::net::TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    let message = String::from_utf8_lossy(&buffer[0..n]);
    println!("Received message: {}", message);
    let response = format!("Hello, {}!", message.trim_end());
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
