/*!
# 线程间通信:互斥锁和读写锁

## [1. 互斥锁](./fn.f03_01_mutex.html)
## [2. 读写锁](./fn.f03_02_rwlock.html)
## [3. 死锁](./fn.f03_03_dead_lock.html)
*/

/**
# 互斥锁
1. 并发模型中，互斥锁用于控制多个线程对共享资源互斥访问的信号量，也就是说互斥锁用来避免多个线程同时访问一个共享资源
2. Rust中提供了std::sync::Mutex类型来实现互斥锁
    1. Mutex::new创建一个互斥锁，包裹要使用的共享资源
    2. lock方法，尝试锁住共享资源，返回一个Result，Result::Ok包裹共享资源的引用
    3. 当锁离开作用域drop之后自动解锁
    4. 通常要结合Arc进行使用，通过多线程安全的引用计数使得锁可以被传递到不同线程中
    5. Mutex和RefCell一样支持修改内部的值，在lock成功之后返回的类型是MutexGuard这样的智能指针，可以通过`*`运算符解引用使用
    ```rust
    use std::sync::{Arc, Mutex};
    use std::thread;
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            if let Ok(mut num) = counter.lock() {
                *num += 1;
                println!("in thread {i}, counter is {} now", *num);
            };
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    ```
3. lock的注释很明确指明了该方法会阻塞当前的线程直到能够锁住
*/
pub fn f03_01_mutex() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            if let Ok(mut num) = counter.lock() {
                *num += 1;
                println!("in thread {i}, counter is {} now", *num);
            };
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

/**
# 读写锁
1. 和互斥锁相比，读写锁允许同时存在多个读取共享资源的线程或者一个写入共享资源的线程
2. Rust中提供std::sync::RwLock作为读写锁
3. RwLock::new包裹要共享的资源
4. read和write两个方法用以获取到对应的智能指针
5. 同样要结合线程安全的Arc使用
    ```rust
    let counter = Arc::new(RwLock::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            if i % 2 == 0 && i > 5 {
                let counter = counter.write();
                let Ok(mut counter) = counter else {
                    return;
                };
                println!("写入线程{i}执行中...");
                *counter += 1;
                println!("写入线程{i}写入完毕!");
            } else {
                let counter = counter.read();
                let Ok(counter) = counter else {
                    return;
                };
                println!("读取线程{i}执行中...");
                println!("当前值为:{}", *counter);
                thread::sleep(Duration::from_millis(2000));
                println!("读取线程{i}读取完毕!");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    ```
*/
pub fn f03_02_rwlock() {
    use std::sync::{Arc, RwLock};
    use std::thread;
    use std::time::Duration;

    let counter = Arc::new(RwLock::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            if i % 2 == 0 && i > 5 {
                let counter = counter.write();
                let Ok(mut counter) = counter else {
                    return;
                };
                println!("写入线程{i}执行中...");
                *counter += 1;
                println!("写入线程{i}写入完毕!");
            } else {
                let counter = counter.read();
                let Ok(counter) = counter else {
                    return;
                };
                println!("读取线程{i}执行中...");
                println!("当前值为:{}", *counter);
                thread::sleep(Duration::from_millis(2000));
                println!("读取线程{i}读取完毕!");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/**
# 死锁
1. 通过互斥锁演示循环等待导致死锁的情况
2. 单线程的情况
    ```rust
    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
    ```
3. 多线程死锁的情况
    1. 假设有两个资源data_1，data_2
    2. 线程_1需要先获取data_1才能去获取data_2
    3. 线程_2需要先获取data_2才能去获取data_1
    4. 这种情况下就可能去循环等待对方释放
    5. 使用try_lock可以一定程度解决这种问题，try_lock锁不住会返回一个错误而不是阻塞线程
    ```rust
    let data_1 = Arc::new(Mutex::new(0));
    let data_2 = Arc::new(Mutex::new(0));
    let handle_1;
    {
        let data_1 = data_1.clone();
        let data_2 = data_2.clone();
        handle_1 = thread::spawn(move || {
            // 先锁住1
            println!("线程1锁住data_1");
            let mut data_1 = data_1.lock().unwrap();
            *data_1 += 1;
            // 模拟循环等待，先sleep让另外一个线程锁住2
            thread::sleep(Duration::from_millis(500));
            println!("等待线程2释放data_2的锁");
            let _data_2 = data_2.lock().unwrap();
        });
    }
    let handle_2;
    {
        let data_1 = data_1.clone();
        let data_2 = data_2.clone();
        handle_2 = thread::spawn(move || {
            // 先锁住2
            println!("线程2锁住data_2");
            let mut data_2 = data_2.lock().unwrap();
            *data_2 += 1;
            // 模拟循环等待，先sleep让另外一个线程锁住1
            thread::sleep(Duration::from_millis(500));
            println!("等待线程2释放data_1的锁");
            let _data_1 = data_1.lock().unwrap();
        });
    }
    handle_1.join().unwrap();
    handle_2.join().unwrap();
    ```
*/
pub fn f03_03_dead_lock() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    let data_1 = Arc::new(Mutex::new(0));
    let data_2 = Arc::new(Mutex::new(0));
    let handle_1;
    {
        let data_1 = data_1.clone();
        let data_2 = data_2.clone();
        handle_1 = thread::spawn(move || {
            // 先锁住1
            println!("线程1锁住data_1");
            let mut data_1 = data_1.lock().unwrap();
            *data_1 += 1;
            // 模拟循环等待，先sleep让另外一个线程锁住2
            thread::sleep(Duration::from_millis(500));
            println!("等待线程2释放data_2的锁");
            let _data_2 = data_2.lock().unwrap();
        });
    }
    let handle_2;
    {
        let data_1 = data_1.clone();
        let data_2 = data_2.clone();
        handle_2 = thread::spawn(move || {
            // 先锁住2
            println!("线程2锁住data_2");
            let mut data_2 = data_2.lock().unwrap();
            *data_2 += 1;
            // 模拟循环等待，先sleep让另外一个线程锁住1
            thread::sleep(Duration::from_millis(500));
            println!("等待线程2释放data_1的锁");
            let _data_1 = data_1.lock().unwrap();
        });
    }
    handle_1.join().unwrap();
    handle_2.join().unwrap();
}
