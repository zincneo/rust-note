use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:21001")?;
    let message = "Rust Tcp Message";
    stream.write(message.as_bytes())?;
    stream.flush()?;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    let received_message = String::from_utf8_lossy(&buffer[..]);
    println!("Received message from server: {}", received_message);
    Ok(())
}
