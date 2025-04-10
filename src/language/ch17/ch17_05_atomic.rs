/*!
# 线程间通信:原子操作

- 原子指的是一系列不可被 CPU 上下文交换的机器指令，这些指令组合在一起就形成了原子操作
- 在多核 CPU 下，当某个 CPU 核心开始运行原子操作时，会先暂停其它 CPU 内核对内存的操作，以保证原子操作不会被其它 CPU 内核所干扰
- 由于原子操作是通过指令提供的支持，因此它的性能相比锁和消息传递会好很多
- 相比较于锁而言，原子类型不需要开发者处理加锁和释放锁的问题，同时支持修改，读取等操作，还具备较高的并发性能，几乎所有的语言都支持原子类型

## [1. 使用Atomic作为全局变量](./fn.f05_global_var.html)

## 2. Atomic应用场景

- 无锁(lock free)数据结构
- 全局变量，例如全局自增 ID
- 跨线程计数器，例如可以用于统计指标
*/

use std::ops::Sub;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Instant;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;
static R: AtomicU64 = AtomicU64::new(0);
static MR: Mutex<u64> = Mutex::new(0);

/**
# 使用Atomic作为全局变量
- 这属于原子类型的常见使用场景
- Rust提供的原子类型在std::sync::atomic模块下
- 下面的示例对比使用原子类型的AtomicU64开十个线程加10000000次和使用Mutex加10000000耗时差距
- 和Mutex相同，具备内部可变性，不需要声明为mut
```
const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;
static R: AtomicU64 = AtomicU64::new(0);

fn main() {
    fn add_n_times(n: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..n {
                R.fetch_add(1, Ordering::Relaxed);
            }
        })
    }

    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("使用原子类型耗时: {:?}", Instant::now().sub(s));

    let s = Instant::now();
    // 这里也可以使用全局变量static的Mutex而不使用Arc，性能不会有什么差别
    let num = Arc::new(Mutex::new(0));
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        let num = num.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..N_TIMES {
                let mut num = num.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, *num.lock().unwrap());
    println!("使用互斥锁耗时: {:?}", Instant::now().sub(s));

    // windows里面套wsl能差十倍性能
}
```
*/
pub fn f05_global_var() {
    fn add_n_times(n: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..n {
                R.fetch_add(1, Ordering::Relaxed);
            }
        })
    }

    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("使用原子类型耗时: {:?}", Instant::now().sub(s));

    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(thread::spawn(move || {
            for _ in 0..N_TIMES {
                let mut num = MR.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, *MR.lock().unwrap());
    println!("使用互斥锁耗时: {:?}", Instant::now().sub(s));
}
