#![allow(dead_code)]
#![allow(unused)]

/**
# 条件变量
- 在并发编程中通常配合互斥锁实现挂起的线程满足了预定的条件之后再执行
- std::Condvar::new创建一个条件变量
- Condcar::wait方法
    - 传入一个MutexGuard类型的变量
    - 暂时释放这个锁并阻塞当前线程
    - 让其它线程可以获取锁更新数据
    - 其他地方调用了Condvar::notify方法，恢复锁并唤醒线程
        - notify_one 只通知被这个条件变量锁住的线程
        - notify_all 通知所有具有该条件变量的线程
*/
pub fn f01_condvar() {
    // 使用条件变量+互斥锁实现主线程和子线程交替执行
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;

    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = thread::spawn(move || {
        let mut lock = cflag.lock().unwrap(); // 在这里对互斥锁flag进行上锁，其他线程要获取锁需要等该锁被释放
        let mut counter = 0;
        while counter < 3 {
            while !*lock {
                // 使用条件变量的wait方法暂时释放锁
                // 同时wait方法会阻塞当前线程，直到其他地方调用了条件变量的notify方法恢复锁并唤醒
                lock = ccond.wait(lock).unwrap();
            }
            *lock = false;
            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        thread::sleep(std::time::Duration::from_millis(500));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
}
