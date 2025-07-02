use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 通过模拟两个线程先后锁住资源1，2等待对方释放锁来制造死锁
    let resource_1 = Arc::new(Mutex::new(0));
    let resource_2 = Arc::new(Mutex::new(0));

    let thread_1;
    let thread_2;
    {
        let resource_1 = resource_1.clone();
        let resource_2 = resource_2.clone();
        thread_1 = thread::spawn(move || {
            // 先锁住资源1的情况在尝试去锁住资源2
            let mut resource_1 = resource_1.lock().unwrap();
            // 休眠一定时间，让资源2被另外一个线程锁住
            println!("线程1锁住了资源1，需要再锁住资源2才能继续执行");
            thread::sleep(Duration::from_millis(500));
            let mut resource_2 = resource_2.lock().unwrap();
            // 锁住2的情况下才去使用1,2
            *resource_1 += 10;
            *resource_2 += 10;
            println!("线程1已经使用完毕，立刻释放锁");
        });
    }

    {
        let resource_1 = resource_1.clone();
        let resource_2 = resource_2.clone();
        thread_2 = thread::spawn(move || {
            // 先锁住资源2的情况在尝试去锁住资源1
            let mut resource_2 = resource_2.lock().unwrap();
            println!("线程2锁住了资源2，需要再锁住资源1才能继续执行");
            // 休眠一定时间，让资源2被另外一个线程锁住
            thread::sleep(Duration::from_millis(500));
            let mut resource_1 = resource_1.lock().unwrap();
            // 锁住2的情况下才去使用1,2
            *resource_1 += 10;
            *resource_2 += 10;
            println!("线程2已经使用完毕，立刻释放锁");
        });
    }

    thread_1.join().unwrap();
    thread_2.join().unwrap();
}
