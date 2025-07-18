// 并发编程模拟在终端输入一行，创建一个客户端连接，发送消息到服务端
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, spawn};

fn main() -> thread::Result<()> {
    let server = spawn(server);

    let client = spawn(client);

    if let Err(e) = server.join()? {
        println!("server error: {:?}", e);
    };
    if let Err(e) = client.join()? {
        println!("client error: {:?}", e);
    }
    Ok(())
}

fn server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:21001")?;
    let mut count = 0;
    for stream in listener.incoming() {
        count += 1;
        spawn(move || -> io::Result<()> {
            println!("开始处理第{count}个连接");
            let stream = stream?;
            handle_server_stream(stream)?;
            Ok(())
        });
    }
    Ok(())
}

fn client() -> io::Result<()> {
    let input = io::stdin();
    let mut buf = String::from("");
    while let Ok(n) = input.read_line(&mut buf)
        && n != 0
    {
        println!("客户端从标准输入读入了: {:?}", buf);
        handle_client_stream(&buf)?;
        buf.clear();
    }
    Ok(())
}

fn handle_server_stream(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    while let Ok(n) = stream.read(&mut buf[..])
        && n != 0
    {
        println!("接收到: {:?}", String::from_utf8_lossy(&buf[0..n]));
    }
    Ok(())
}

fn handle_client_stream(content: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:21001")?;
    stream.write(content.as_bytes())?;
    Ok(())
}
