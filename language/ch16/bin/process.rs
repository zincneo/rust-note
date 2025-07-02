// 第三方fork库的封装
use fork::{Fork, fork};

fn main() -> Result<(), i32> {
    // 与c中使用fork函数相同，但返回值是Result类型
    // 使用?传播错误
    let pid = fork()?;
    match pid {
        Fork::Parent(_child) => {
            // 进程空间之间互相隔离，不影响其他进程执行
            let pid = std::process::id();
            println!("父进程的进程空间会执行这个代码块，进程id{pid}");
        }
        Fork::Child => {
            // sleep可以看到就算父进程执行结束，子进程任然会继续执行，fork之后管理进程的已经是内核调度了，不同进程之间要同步和通信只能依赖IPC通信
            std::thread::sleep(std::time::Duration::from_secs(2));
            let pid = std::process::id();
            println!("子进程的进程空间会执行这个代码块, 进程id{pid}");
        }
    };
    Ok(())
}
