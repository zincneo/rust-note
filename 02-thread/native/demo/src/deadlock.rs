/**
# 死锁
- 并发编程中由于共享资源被不同线程竞争导致的线程互相等待对方释放需要的资源权限使得都无法继续执行的状态称为死锁
- 简单的一个利用互斥锁制造死锁的例子(A/B两个互斥锁)(线程1获取顺序A、B)(线程2获取顺序B、A)
*/
pub fn run() {
    use std::sync::{Arc, Mutex};
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));

    let t1;
    {
        let (a, b) = (a.clone(), b.clone());
        t1 = spawn(move || {
            let _a = a.lock();
            println!("thread-1 lock a, wait b");
            sleep(Duration::from_millis(200));
            let _b = b.lock();
        });
    }
    let t2;
    {
        let (a, b) = (a.clone(), b.clone());
        t2 = spawn(move || {
            let _b = b.lock();
            println!("thread-2 lock b, wait a");
            sleep(Duration::from_millis(200));
            let _a = a.lock();
        });
    }
    let _ = t1.join();
    let _ = t2.join();
    println!("thread-1and2 end");
}
