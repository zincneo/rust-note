// 1. 使用mini_redis连接本地缓存
use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("the result from the server is {:?}", result);

    Ok(())
}

// #[tokio::main]做以下转换
/*
#[tokio::main]
async fn main() {
    println!("hello");
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}
*/
