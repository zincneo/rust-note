/**
# rust标准库线程
1. 和系统线程是1:1线程关系
2. 标准库模块thread提供创建spawn方法和休眠sleep方法
3. 创建线程的返回值通过join方法在父线程进行阻塞等待子线程执行完毕
4. 标准库模块process提供id方法可以获取当前进程id
*/
pub fn run() {
    use std::process::id;
    use std::thread::{sleep, spawn};
    use std::time::Duration;
    let mut handles = vec![];

    for i in 0..5 {
        handles.push(spawn(move || {
            for _ in 0..5 {
                println!(
                    "{i} thread, process id {}, thread id: {:?}",
                    id(),
                    std::thread::current().id()
                );
                sleep(Duration::from_millis(2000));
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}
