#![allow(dead_code)]
#![allow(unused)]

use futures::future;

/**
# async/await
- 异步编程本质底层还是基于线程实现，但是可以将多个任务映射到少量线程上
    - CPU密集型的任务
        1. 少数任务，长期占据CPU运行:如并行计算
        2. 不需要高并发能力，反而是不切换线程，甚至将线程直接绑定到指定上核心上
        3. 这种需求使用标准库的os 1:1线程模型更好
    - IO密集型的任务
        1. 数量庞大的任务:如web服务器
        2. 每个任务对CPU的占用其实很少大部分时间在等待IO
        3. 这种需求使用异步编程可以将多个任务转换为内存中任务的切换而不是线程切换
- 语言提供的async, awiat关键字
- async标记的代码块(函数或者{})会返回一个实现Future特征的类型，该类型需要在同步代码中被异步执行器进行调用，否则什么也不会发生
- 在异步代码中调度其他异步代码可以使用.await等待另外一个异步代码执行完毕再继续向后执行
- 标准库目前还没有提供异步执行器，这里使用futures库提供的异步执行器
    1. futures::executor::block_on 阻塞当前同步代码直到Future执行完毕
    2. futures::join! 阻塞当前异步代码直到传入的Future执行完毕，将所有Future的返回值包裹在元组内返回
    3. futures::try_jon! 阻塞当前的异步代码直到传入Future执行完毕，如果有一个Err就返回Error并且结束后续Future执行，否则将所有Ok包装在元组内返回
*/
pub fn f01_async_awiat() {
    async fn do_anything(val: i32) {
        std::thread::sleep(std::time::Duration::from_millis(200));
        println!("eat eat eat!");
        println!("----{val}----");
    }
    async fn do_something() {
        // 异步函数中调用另外一个异步函数，使用.await等待另外一个函数执行完毕才向后执行
        do_anything(0).await;
        println!("go go go!");
        futures::join!(
            do_anything(1),
            do_anything(2),
            do_anything(3),
            do_anything(4),
            do_anything(5)
        );
        let a = async { 1 };
        let b = async { 2 };
        let c = async { 3 };
        let t = futures::join!(a, b, c);
        println!("{:?}", t);
        let closure = |val| async move {
            println!("{:?}", val);
            Err::<char, char>(val)
        };
        let a = closure('a');
        let b = closure('b');
        futures::try_join!(a, b);
    }
    let future = do_something();
    futures::executor::block_on(future);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_async_await() {
        assert_eq!(f01_async_awiat(), ());
    }
}
