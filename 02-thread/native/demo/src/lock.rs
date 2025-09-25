pub fn run() {
    use std::sync::{Arc, Mutex};
    use std::thread::{current, sleep, spawn};
    use std::time::Duration;
    // Mutex 互斥锁，lock是阻塞方法会等待到其他地方都释放锁然后获取，try_lock是非阻塞的
    // Mutex 没有实现Clone特征，因此要共享给其他线程需要使用Arc
    let count = Arc::new(Mutex::new(1));
    {
        let count = count.clone();
        spawn(move || {
            sleep(Duration::from_millis(200));
            let mut count = count.lock().unwrap();
            *count += 5;
        });
    }
    {
        let count = count.clone();
        spawn(move || {
            sleep(Duration::from_millis(200));
            let mut count = count.lock().unwrap();
            *count += 1;
        });
    }
    sleep(Duration::from_millis(200));
    let mut count = count.lock().unwrap();
    *count += 2;
    println!("{:?}", count);

    // 读写锁，和互斥锁的差别在于可以同时支持多个读或者一个写
    use std::sync::RwLock;

    let count = Arc::new(RwLock::new(1));

    {
        let count = count.clone();
        spawn(move || {
            let count = count.read().unwrap();
            for i in 0..4 {
                println!(
                    "{i} read in thread id {:?}, value {:?}",
                    current().id(),
                    count
                );
                sleep(Duration::from_millis(200));
            }
        });
    }
    {
        let count = count.clone();
        spawn(move || {
            let count = count.read().unwrap();
            for i in 0..4 {
                println!(
                    "{i} read in thread id {:?}, value {:?}",
                    current().id(),
                    count
                );
                sleep(Duration::from_millis(200));
            }
        });
    }
    // let mut count = count.write().unwrap();
    // *count += 5;
    sleep(Duration::from_millis(2000));
}
