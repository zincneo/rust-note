/**
# 互斥锁
- 标准库的互斥锁是操作系统原生互斥锁的封装(Linux上的pthread mutex,windows上的SRWLock封装)
- parking_lot则是Rust原生实现的用户态的锁
- 通过自旋+停车/唤醒机制进入避免频繁进入内核态
- parking_lot的互斥锁没有实现中毒保护
    - 中毒保护是指当获取锁的线程panic的时候锁不会自动释放，再其他地方继续获得会得到一个中毒错误
    - 标准库实现了中毒保护而parking_lot提供的则会在线程panic的时候正常释放锁
- 性能上比标准库提供的更优秀
- 更加节省内存
*/
pub fn run_mutex() {
    use std::sync::Arc;
    use std::thread::spawn;
    use std::time::Instant;
    // 1. 标准库的互斥锁
    {
        use std::sync::Mutex;

        let mutex = Arc::new(Mutex::new(0));

        let prev = Instant::now();
        for _ in 0..1000_000_0 {
            {
                if let Ok(mut val) = mutex.lock() {
                    *val += 1;
                }
            }
        }
        let last = Instant::now();
        println!("std::sync::Mutex duration: {:?}", last - prev);
        {
            let mutex = mutex.clone();
            let handle = spawn(move || {
                let mut val = mutex.lock().unwrap();
                *val += 1;
                panic!("std::sync::Mutex panic");
            });
            let _ = handle.join();
        }
        println!("std::sync::Mutex status: {:?}", mutex.lock());
    }

    // 2. parking_lot的互斥锁
    {
        use parking_lot::Mutex;

        let mutex = Arc::new(Mutex::new(0));

        let prev = Instant::now();
        for _ in 0..1000_000_0 {
            {
                let mut val = mutex.lock();
                *val += 1;
            }
        }
        let last = Instant::now();
        println!("parking_lot::Mutex duration: {:?}", last - prev);
        {
            let mutex = mutex.clone();
            let handle = spawn(move || {
                let mut val = mutex.lock();
                *val += 1;
                panic!("parking_lot::Mutex panic");
            });
            let _ = handle.join();
        }
        println!("parking_lot::Mutex status: {:?}", mutex.lock());
    }
}

/**
# 读写锁
- 和标准库的区别和读写锁类似，同样是系统原生封装和Rust算法实现的差异以及线程崩溃时是否实现中毒的差异
- 除此之外还提供了更多的API，例如锁定超时，将读锁升级为写锁，写锁降级为读锁的方法
- 内部提供读写竞争队列，实现更优秀的读写竞争机制，避免标准库使用系统原生读写锁在有些平台上可能出现的读/写饿死现象
- 更加节省内存
*/
pub fn run_rwlock() {
    use parking_lot::RwLock;
    use std::sync::Arc;
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    let val = Arc::new(RwLock::new(1));
    {
        let val = val.clone();
        spawn(move || {
            let val = val.read();
            println!("{:?}", val);
            sleep(Duration::from_millis(1000));
        });
        sleep(Duration::from_millis(200));
    }
    let val = val.try_write_for(Duration::from_millis(500));
    println!("{:?}", val);
}
