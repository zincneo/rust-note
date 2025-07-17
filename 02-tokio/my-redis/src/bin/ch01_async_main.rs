/*
# 异步运行时以及创建异步任务

## tokio::main宏

该宏用来标记主函数，将主函数中的代码块自动转换到异步运行时中

```rust
#[tokio::main]
async fn main() {
    println!("hello");
}
```

```rust
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}
```

## tokio::spawn

该方法接收一个Future，创建一个异步任务，返回值是一个JoinHanle类型，通过.await阻塞当前任务到异步任务结束

*/
#[tokio::main]
async fn main() {
    println!("hello");

    let handle = tokio::spawn(async move {
        println!("hello async");
    });

    handle.await.unwrap();
}
