/**
## 使用多线程
- std::thread::spawn
  - 这个函数创建一个新的线程并执行
  - spawn的参数是一个闭包(实现FnOnce特征且参数列表为空)
  - spawn不会阻止main线程执行，如果main线程执行完毕则所有线程都停止
  - 千万不要依赖线程的**执行顺序**
  - spawn的返回值类型有一个join方法，这个方法会让调用的线程子线程执行结束再继续执行
- move关键字
  - 对于传递给spwan的闭包上使用move关键字可以强制让父线程的值移动到子线程中
- 线程结束
  - main线程结束会强制让其他线程结束
  - 其他父子线程不会因为父线程结束导致子线程结束，线程会自己运行到代码执行完毕然后结束
  - 如果子线程代码执行不完，通常有两种情况
    1. 循环IO读取，绝大部分时间线程在阻塞，网络服务中很常见
    2. 线程任务是一个循环，内部没有任何阻塞，那么就会占用一个cpu跑满直到main线程结束
- 多线程的性能
  - 创建一个线程的开销是不可忽视的，大概需要0.24毫秒，如果已经创建了很多的话还会更耗时
  - 因此真的值得创建线程的情况才会去创建线程
  - 对于网络IO这种大部分时间都在阻塞而且用户可能成千上万的就不要使用1:1线程模型了，使用async/await的M:N并发模型
*/
fn _ch05_01_thread() {
    use std::thread;
    use std::time::Duration;
    // 1. 创建线程
    {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    // 2. 子线程无限循环
    {
        let a = thread::spawn(|| {
            thread::spawn(|| loop {
                println!("I am a new thread");
            })
        });
        a.join().unwrap();
        println!("Child thread is finish!");
        // 睡眠一段时间，看子线程创建的子线程是否还在运行
        thread::sleep(Duration::from_millis(100));
    }
}

/**
## 线程屏障
- 线程屏障可以让多个线程都执行到某个点之后才继续往后执行
*/
fn _ch05_02_barrier() {
    use std::sync::{Arc, Barrier};
    use std::thread;
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for i in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before {} thread wait", i);
            b.wait();
            println!("after {} thread wait", i);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

/**
## 线程局部变量
- 标准库thread_local
  - 每个线程都会拿到初始值作为开始
  - 各个线程之间值不会互相干扰
  - 需要使用static关键字声明为生命周期为'static的静态变量
*/
fn _ch05_03_local_variable() {
    // 1. thread_local
    {
        use std::cell::RefCell;
        use std::thread;

        thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

        // 主线程使用这个局部变量，修改为2
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch05_01() {
        assert_eq!(_ch05_01_thread(), ());
    }

    #[test]
    fn ch05_02() {
        assert_eq!(_ch05_02_barrier(), ());
    }

    #[test]
    fn ch05_03() {
        assert_eq!(_ch05_03_local_variable(), ());
    }
}
