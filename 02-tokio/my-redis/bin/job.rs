use std::rc::Rc;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    for i in 0..10 {
        // 1. spwan API，创建任务异步执行，入参是一个Future，不会阻塞当前的代码，返回值是一个句柄，具有await方法
        // 2. 参数生命周期必须是'static，也就是不允许使用任何外部数据的引用，即使要捕获外部的实现Copy特征的数据也要使用move表示所有权转移到异步代码块内部
        // 3. 参数必须满足Send约束，因为任务在.await执行过程中发生阻塞时，Tokio调度器会将任务在线程间移动
        tokio::spawn(async move {
            sleep(std::time::Duration::from_millis(300)).await;
            println!("{i}");
        });
    }
    sleep(std::time::Duration::from_secs(1)).await;

    async fn foo() {
        println!("foo");
    }
    // async代码块满足Send约束要求在内部调用.await时，持有的数据都实现了Send特征
    // 1. 合法的情况
    tokio::spawn(async move {
        // let rc = Rc::new("hello");
        {
            let rc = Rc::new(1);
            println!("{:?}", rc);
        } // Rc类型没有实现Send特征，但是rc在这里被Drop
        foo().await; // 合法，因为在await产生调度的时候，当前持有的数据满足都实现Send特征的条件
        // 2. 不合法的情况
        // println!("{:?}", rc); // rc的生命周期延续到此处时在await调度的时候持有了没有实现Send特征的数据导致出错
    });
}
