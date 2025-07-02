use fork::{Fork, fork};
use ipc_channel::ipc;

fn main() {
    let Ok((tx, rx)) = ipc::channel::<i32>() else {
        return;
    };

    let pid = fork();
    if let Ok(Fork::Parent(_)) = pid {
        let mut count = 0;
        loop {
            count += 1;
            println!("主进程尝试{count}次发送");
            match tx.send(count) {
                Ok(()) => (),
                Err(e) => println!("{e}"),
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    };
    if let Ok(Fork::Child) = pid {
        let mut count = 0;
        loop {
            count += 1;
            println!("子进程尝试{count}次接收");
            match rx.recv() {
                Ok(num) => println!("接收到{num}"),
                Err(e) => println!("{e}"),
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    };
}
