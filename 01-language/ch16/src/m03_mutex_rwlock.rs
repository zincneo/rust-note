#![allow(dead_code)]
#![allow(unused)]

/**
# 互斥锁
- 单线程希望让多个所有者持有一份数据，并且同时具有一个运行时检查的可变引用可以使用Rc+RefCell
- 并发模型中，互斥锁用于控制多个线程对共享资源互斥访问，也就是说互斥锁用来避免多个线程同时访问一个共享资源
- 多线程下实现这样的需求可以使用Arc+std::sync::Mutex
- Mutex是用来实现互斥访问并提供内部可变性的智能指针
    1. Mutex::new创建一个互斥锁，包裹要使用的共享资源
    2. lock方法，尝试锁住共享资源，返回一个Result，Result::Ok包裹共享资源的引用
    3. 当锁离开作用域drop之后自动解锁
    4. 通常要结合Arc进行使用，通过多线程安全的引用计数使得锁可以被传递到不同线程中
    5. Mutex和RefCell一样支持修改内部的值，在lock成功之后返回的类型是MutexGuard这样的智能指针，可以通过`*`运算符解引用使用
*/
pub fn f01_mutext() {
    use std::sync::{Arc, Mutex};
    use std::thread::spawn;
    let count = Arc::new(Mutex::new(0));
    let thread_closure = |num, count: Arc<Mutex<i32>>| {
        spawn(move || {
            {
                let count = count.lock(); // 一直阻塞线程直到锁住
                if let Ok(mut count) = count {
                    println!("第{}个线程", num);
                    *count += 1;
                    println!("{}", *count);
                }
            }
            println!("已经释放{num}线程的锁");
        })
    };
    let mut handles = Vec::new();
    for i in 0..10 {
        handles.push(thread_closure(i, count.clone()));
    }
    for handle in handles {
        handle.join();
    }
}

/**
# 读写锁
1. 和互斥锁相比，读写锁允许同时存在多个读取共享资源的线程或者一个写入共享资源的线程
2. Rust中提供std::sync::RwLock作为读写锁
3. RwLock::new包裹要共享的资源
4. read和write两个方法用以获取到对应的智能指针
5. 同样要结合线程安全的Arc使用
*/
pub fn f02_rwlock() {
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
