/*!
# Rust异步编程

## 0. 异步编程

- 异步编程是一个并发编程模型
    - 对比Rust提供的OS线程模型更加适合IO密集的高并发任务，不适合CPU密集的任务
- 目前主流语言基本都支持
- 异步编程允许同时并发运行大量的任务
    - 仅仅需要几个甚至一个 OS 线程或 CPU 核心
    - 现代化的异步编程在使用体验上跟同步编程也几无区别
- Rust通过语言提供关键结合标准库和第三方库提供的特性和运行时实现异步编程
    - 语言本身提供的async和await关键字
    - 标准库提供的必须特征`Future`、类型和函数
    - 官方开发的`futures`库提供一系列实用类型、宏和函数
    - 第三方库提供的异步运行时`tokio`以及`async-std`

## [1. async/await](./fn.f01_async_await.html)
## [2. 同时启动多个异步任务](./fn.f02_join.html)
*/

/**
# async/await

1. 通过async关键字声明函数为异步函数
    ```rust
    async fn test() {}
    ```
2. 异步函数调用不会执行，而是返回一个`impl Future`
3. 同步代码中需要通过一个执行器进行运行
    ```rust
    use futures::executor::block_on;
    let future = test();
    block_on(future);
    ```
4. 异步代码中运行另外一个异步函数使用`.await`
    ```rust
    async fn f1() {}
    async fn f2() {
        f1().await; // 等待异步函数执行完毕才会继续执行，但不会阻塞线程
    }
    ```
*/
pub fn f01_async_await() {
    async fn do_something() {
        do_anything().await;
        println!("go go go !");
    }
    async fn do_anything() {
        println!("other async function");
    }
    let future = do_something();
    use futures::executor::block_on;
    block_on(future);
}

/**
# 同时启动多个异步任务
- futures库提供了`join!`宏，该宏接收可变数量的`impl Future`，并发执行它们
- 该宏需要在异步环境下使用
    ```rust
    async fn f1_01() {
        println!("f1_01");
    }
    async fn f1_02() {
        f1_01().await;
        println!("f1_02");
    }
    async fn f2_01() {
        println!("f2_01");
    }
    async fn f2_02() {
        f2_01().await;
        println!("f2_02");
    }
    async fn async_main() {
        futures::join!(f1_02(), f2_02());
    }
    futures::executor::block_on(async_main());
    ```
*/
pub fn f02_join() {
    async fn f1_01() {
        println!("f1_01");
    }
    async fn f1_02() {
        f1_01().await;
        println!("f1_02");
    }
    async fn f2_01() {
        println!("f2_01");
    }
    async fn f2_02() {
        f2_01().await;
        println!("f2_02");
    }
    async fn async_main() {
        futures::join!(f1_02(), f2_02());
    }
    futures::executor::block_on(async_main());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch21_01() {
        assert_eq!(f01_async_await(), ());
    }

    #[test]
    fn ch21_02() {
        assert_eq!(f02_join(), ());
    }
}
