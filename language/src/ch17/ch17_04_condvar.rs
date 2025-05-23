/*!
# 条件变量
- 在并发编程中条件变量通常是配合互斥锁实现挂起线程满足某些条件之后再继续执行
- std::Condvar::new创建一个条件变量
- Condcar::wait方法
    - 传入一个MutexGuard类型的变量
    - 暂时释放这个锁并阻塞当前线程
    - 让其它线程可以获取锁更新数据
    - 其他地方调用了Condvar::notify方法，恢复锁并唤醒线程
- 示例通过条件变量+互斥锁实现main线程和子线程交替执行
    ```rust
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;
        while counter < 3 {
            while !*lock {
                // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
                // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
                lock = ccond.wait(lock).unwrap();
            }
            *lock = false;
            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        // 等待能够锁住的时候(锁没有被锁住，或者通过条件变量暂时释放的时刻可以获取)
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        // 通知别处锁还回路了，解除阻塞继续执行
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
    ```
*/

pub fn f04_condvar() {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;
        while counter < 3 {
            while !*lock {
                // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
                // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
                lock = ccond.wait(lock).unwrap();
            }
            *lock = false;
            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
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
