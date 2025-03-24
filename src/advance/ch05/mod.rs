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

/**
## 消息通道
- 标准库std::sync::mpsc提供了多发送者单接收者的消息通道
- 该类型的关联方法new会返回一个元组(发送者,接收者)
  - 发送者可以使用send方法发送数据
  - 接收者可以使用recv方法阻塞当前线程，直到收到数据继续执行
  - send和recv方法都会得到一个Result类型
- 接收者可以使用try_recv方法
  - 该方法不会阻塞线程，没有获取到消息的时候会返回一个错误
- 传输数据的规则
  - 发送的数据实现了Copy特征，会发送一份复制
  - 发送的数据没有实现Copy特征，会发生所有权转移
- for循环语法糖可以将接收者转换为一个迭代器，接收消息直到发送者都被drop
- 发送者可以clone多份，实现在不同线程中发送
- 通道满足先进先出的原则，发送的顺序和接收的顺序一致
- mpsc提供异步通道和同步通道
  - channel使用的是异步通道，无论接收者是否正在接收消息，消息发送者在发送消息时都不会阻塞
  - sync_channel使用的是同步通道，发送消息是阻塞的，只有在消息被接收后才解除阻塞
    - sync_channel接收一个参数，表示消息缓存的数量，当缓存队列满时才会阻塞当前线程
- 当所有发送者被drop或者所有接收者drop后，通道自动关闭
*/
fn _ch05_04_mpsc() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    // 1. 单接收者，单发送者
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            // 发送一个数字1，编译器自动推导发送者和接收者的泛型参数列表接收一个i32的参数
            tx.send(1).unwrap();
        });

        // 主线程中使用recv方法阻塞并等待接收
        println!("receive {}", rx.recv().unwrap());
    }

    // 2. try_recv不阻塞
    {
        let (tx, rx) = mpsc::channel();

        let t = thread::spawn(move || {
            tx.send(1).unwrap();
        });

        println!("receive {:?}", rx.try_recv());
        println!("receive {:?}", rx.try_recv());
        println!("receive {:?}", rx.try_recv());
        t.join().unwrap();
        println!("receive {:?}", rx.try_recv());
        println!("receive {:?}", rx.try_recv());
    }

    // 3. for语法糖接收
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    // 4. 多发送者
    {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            tx.send(String::from("hi from raw tx")).unwrap();
        });

        thread::spawn(move || {
            tx1.send(String::from("hi from cloned tx")).unwrap();
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    // 5. 同步通道
    // 发送消息会阻塞当前线程，直到消息被接收者接收
    {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::spawn(move || {
            println!("发送之前");
            tx.send(1).unwrap();
            // 发送之后阻塞当前线程
            // 接收者接收send的消息之后线程继续执行
            println!("发送之后");
        });

        println!("睡眠之前");
        thread::sleep(Duration::from_secs(3));
        println!("睡眠之后");

        println!("receive {}", rx.recv().unwrap());
        handle.join().unwrap();
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

    #[test]
    fn ch05_04() {
        assert_eq!(_ch05_04_mpsc(), ());
    }
}
