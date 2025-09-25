pub fn run() {
    // crossbeam提供bounded/unbounded两种对应标准库中的同步和异步通道
    use crossbeam::channel::unbounded;
    use std::thread::{self, sleep, spawn};
    use std::time::Duration;
    // crossbeam中的通道默认是mpmc的，多发送者多接收者，如果多收的话就竞争获取，先获取的拿走数据后来的就拿不到这一条数据
    // crossbeam的通道除了阻塞的send、recv API也提供非阻塞的try_send、try_recv API还有限定时间超时拿不到或者发送不了返回Err的send_timeout、recv_timeout方法
    let (tx, rx) = unbounded::<i32>();
    let mut handles = vec![];
    {
        let tx = tx.clone();
        let rx = rx.clone();
        handles.push(spawn(move || {
            let _ = tx.send(1);
            sleep(Duration::from_millis(2000));
            let _ = tx.send(2);
        }));

        handles.push(spawn(move || {
            for val in rx {
                println!(
                    "thread ID: {:?}, reciver value: {}",
                    thread::current().id(),
                    val
                );
            }
        }));
    }

    {
        let tx = tx.clone();
        let rx = rx.clone();
        handles.push(spawn(move || {
            let _ = tx.send(3);
            sleep(Duration::from_millis(2000));
            let _ = tx.send(4);
        }));

        handles.push(spawn(move || {
            for val in rx {
                println!(
                    "thread ID: {:?}, reciver value: {}",
                    thread::current().id(),
                    val
                );
            }
        }));
    }
    // 提前回收掉主线程中发送者，减少计数
    std::mem::drop(tx);
    for handle in handles {
        let _ = handle.join();
    }
}

/**
# select宏
- crossbeam提供的宏，允许同时等待多个通道操作，执行第一个准备好的操作
- select宏还可以配合after、tick、never使用
    1. after `recv(after(Duration))`接收到一个延时
    2. tick `recv(tick(Duration))`和after的区别在于会按Duration周期性接收
    3. never `recv(never)`永远不会触发
*/
pub fn run_select() {
    use crossbeam::{
        channel::{after, bounded, tick, unbounded},
        select,
    };
    use std::thread::spawn;
    use std::time::Duration;
    let (t1, r1) = bounded(0);
    let (t2, r2) = bounded(0);
    let (t3, r3) = unbounded();
    {
        let t1 = t1.clone();
        spawn(move || {
            t1.send("sender 1").unwrap();
        });
    }
    {
        let t2 = t2.clone();
        spawn(move || {
            t2.send("sender 2").unwrap();
        });
    }
    {
        let t3 = t3.clone();
        spawn(move || {
            t3.send(3).unwrap();
        });
    }
    for _ in 0..3 {
        select! {
            recv(r1) -> msg => {
                match msg {
                    Ok(m) => println!("reciver 1, value {}", m),
                    Err(_) => println!("channel 1 close"),
                }
            }
            recv(r2) -> msg => {
                println!("{:?}", msg);
            }
            recv(r3) -> msg => {
                println!("{:?}", msg);
            }
        }
    }

    let (s, r) = unbounded::<i32>();
    let timeout = Duration::from_millis(100);

    select! {
        recv(r) -> msg => println!("received {:?}", msg),
        recv(after(timeout)) -> _ => println!("timed out"),
    }

    s.send(1).unwrap();

    let timeout = Duration::from_millis(50);

    let mut count = 0;
    for _ in 0..5 {
        select! {
            recv(r) -> msg => println!("received {:?}", msg),
            recv(tick(timeout)) -> _ => {
                println!("{:?}", count);
                count += 1;
            },
        }
    }
}
