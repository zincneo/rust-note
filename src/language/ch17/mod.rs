/*!
# Rust并发编程

## 0. 并发和并行

- 并发(Concurrent): 操作系统控制多个任务轮换执行，对于用户切换的够快就好像同时执行
- 并行(Parallel): 真的有多个处理器同时在处理多个任务
- 实际情况是在现代操作系统中并发并行通常同时存在
- Rust提供的并发编程是基于操作系统线程的1:1模型
- Rust也提供了异步的M:N模型，比如第三方异步库tokio
- 并发编程容易出现的风险
    1. 竞态条件(race conditions)，多个线程以非一致性的顺序访问资源
    2. 死锁(deadlocks)，两个线程都想使用某个资源，但是又都在等待对方释放资源后才能使用，结果最终都无法继续执行
    3. 一些因为多线程导致的很隐晦的 BUG，难以复现和解决

## [1. 使用多线程](./fn.f01_use_thread.html)
## [2. 线程间通信:消息通道](./ch17_02_mpsc/index.html)
## [3. 线程间通信:互斥锁和读写锁](./ch17_03_mutex_rwlock/index.html)
## [4. 线程间通信:条件变量控制执行顺序](./ch17_04_condvar/index.html)
## [5. 线程间通信:原子操作](./ch17_05_atomic/index.html)
## [6. 线程补充:线程屏障](./fn.f06_barrier.html)
## [7. 线程补充:线程局部变量](./fn.f07_local_var.html)
## [8. 线程补充:第三方线程局部变量](./fn.f08_local_var.html)
## [9. 线程补充:只被执行一次的函数](./fn.f09_once.html)
*/

/**
# 使用多线程

- std::thread::spawn用于创建线程
    - 该函数传入一个闭包，闭包将在新线程内执行
    - 该函数返回一个JoinHanle类型的值，该类型提供一个join方法，将阻塞当前线程直到子线程执行结束
    - 如果要将值传递到子线程中需要在闭包上加move关键字强调以所有权转移的方式捕获变量
    ```rust
    use std::thread;
    use std::time::Duration;

    let handle_1 = thread::spawn(|| {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(1000));
            println!("first thread: {i}");
        }
    });

    let handle_2 = thread::spawn(|| {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(1000));
            println!("second thread: {i}");
        }
    });

    for i in 0..3 {
        thread::sleep(Duration::from_millis(500));
        println!("main thread: {i}");
    }

    handle_1.join().unwrap();
    handle_2.join().unwrap();
    ```
- Rust中main线程结束的时候就结束所有线程
- 父线程非main线程的线程在父线程结束的时候不会结束
*/
pub fn f01_use_thread() {
    use std::thread;
    use std::time::Duration;

    let handle_1 = thread::spawn(|| {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(1000));
            println!("first thread: {i}");
        }
    });

    let handle_2 = thread::spawn(|| {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(1000));
            println!("second thread: {i}");
        }
    });

    for i in 0..3 {
        thread::sleep(Duration::from_millis(500));
        println!("main thread: {i}");
    }

    handle_1.join().unwrap();
    handle_2.join().unwrap();
}

pub mod ch17_02_mpsc;
pub mod ch17_03_mutex_rwlock;
pub mod ch17_04_condvar;
pub mod ch17_05_atomic;

/**
# 线程屏障
- std::sync::Barrier提供让多个线程执行到某个点之后一起执行的能力
- Barrier::new传入线程的数量，当调用Barrier::wait方法的线程数量达到new中传入的数量之后继续执行
```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move|| {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```
*/
pub fn f06_barrier() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for i in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("{i} thread before wait");
            b.wait();
            println!("{i} thread after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/**
# 线程局部变量
- 每一个线程能够拿到相同初始化独立的变量，互相不干扰
- 标准库中提供了thread_local宏来初始化线程局部变量
- 每个线程内部通过在该变量的with方法传入闭包使用
```rust
thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

FOO.with(|f| {
    assert_eq!(*f.borrow(), 1);
    *f.borrow_mut() = 2;
});

// 每个线程开始时都会拿到线程局部变量的FOO的初始值
let t = thread::spawn(move|| {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 3;
    });
});

// 等待线程完成
t.join().unwrap();

// 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
FOO.with(|f| {
    assert_eq!(*f.borrow(), 2);
});
```
*/
pub fn f07_local_var() {
    use std::cell::RefCell;
    use std::thread;
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到线程局部变量的FOO的初始值
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    // 等待线程完成
    t.join().unwrap();

    // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

/**
# 第三方库线程局部变量
- thread-local库提供了ThreadLocal类型来创建线程局部变量
- 允许每一个线程有独立的值拷贝
- 而且可以将多个拷贝汇总到一个迭代器中使用
    ```rust
    use std::cell::Cell;
    use std::sync::Arc;
    use std::thread;
    use thread_local::ThreadLocal;

    // 创建线程局部变量
    let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            // 将计数器加1
            // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
            // 这只能在线程退出后发生，因此不会导致任何竞争条件
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }

    // 等待所有子线程结束
    for handle in v {
        handle.join().unwrap();
    }
    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
        // 因为一些线程已退出，并且其他线程会回收退出线程的对象
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });

    // 和为5
    assert_eq!(total, 5);
    ```
*/
pub fn f08_local_var() {
    use std::cell::Cell;
    use std::sync::Arc;
    use std::thread;
    use thread_local::ThreadLocal;

    // 创建线程局部变量
    let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            // 将计数器加1
            // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
            // 这只能在线程退出后发生，因此不会导致任何竞争条件
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }

    // 等待所有子线程结束
    for handle in v {
        handle.join().unwrap();
    }
    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
        // 因为一些线程已退出，并且其他线程会回收退出线程的对象
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });

    // 和为5
    assert_eq!(total, 5);
}

/**
# 只被调用一次的函数
- 有些场景需要某一个函数在多线程环境中只被调用一次
- 标准库提供了std::sync::Once来实现这个功能
- 该类型的实例多次调用call_once方法(参数是一个闭包)，只有第一次会执行
```rust
use std::thread;
use std::sync::Once;

static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn main() {
    let handle1 = thread::spawn(move || {
        // call_once如果被执行，之后的就不会再执行
        INIT.call_once(|| {
            unsafe {
                VAL = 1;
            }
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 2;
            }
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}
*/
pub fn f09_once() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch17_01() {
        assert_eq!(f01_use_thread(), ());
    }

    #[test]
    fn ch17_02() {
        assert_eq!(ch17_02_mpsc::f02_01_use(), ());
        assert_eq!(ch17_02_mpsc::f02_02_try_revc(), ());
        assert_eq!(ch17_02_mpsc::f02_03_sync(), ());
    }

    #[test]
    fn ch17_03() {
        assert_eq!(ch17_03_mutex_rwlock::f03_01_mutex(), ());
        assert_eq!(ch17_03_mutex_rwlock::f03_02_rwlock(), ());
        assert_eq!(ch17_03_mutex_rwlock::f03_03_dead_lock(), ());
    }

    #[test]
    fn ch17_04() {
        assert_eq!(ch17_04_condvar::f04_condvar(), ());
    }

    #[test]
    fn ch17_05() {
        assert_eq!(ch17_05_atomic::f05_global_var(), ());
    }

    #[test]
    fn ch17_06() {
        assert_eq!(f06_barrier(), ());
    }

    #[test]
    fn ch17_07() {
        assert_eq!(f07_local_var(), ());
    }

    #[test]
    fn ch17_08() {
        assert_eq!(f08_local_var(), ());
    }

    #[test]
    fn ch17_09() {
        assert_eq!(f09_once(), ());
    }
}
