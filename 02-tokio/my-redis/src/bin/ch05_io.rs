use tokio::fs;
// AsyncReadExt 读特征 AsyncWriteExt 写特征
// 提供的方法和标准库中同步的相同，只不过都是异步的版本
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = fs::File::open("foo.txt").await?;
    let mut buf = [0; 10];
    // 每次读入10个Byte的数据
    // 当读入的字节为0时说明已经读到了文件末尾
    while let Ok(n) = f.read(&mut buf[..]).await
        && n != 0
    {
        println!("{n}: {:?}", buf);
    }

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("bar.txt")
        .await?;

    let n = f.write(b"some bytes").await?;
    println!("{n}");

    Ok(())
}
