use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep, spawn}, time::Duration,
};

type Reciver<T> = Arc<Mutex<mpsc::Receiver<T>>>;
type Job = Box<dyn FnOnce() + Send + 'static>;
struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0); // 线程数量<=0直接让程序panic
        let (sender, reciver) = mpsc::channel();
        let sender = Some(sender);
        let reciver = Arc::new(Mutex::new(reciver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, reciver.clone()));
        }
        Self { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    // Option包裹是为了在Drop特征中可以解决使用join销毁线程时导致的移动问题(take方法拿走Some包裹的内容留下None)
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciver: Reciver<Job>) -> Self {
        let thread = Some(spawn(move || 
            // 这里在外部使用loop，而没有使用while let直接去获得lock，是因为如果使用while let那么将会导致lock在离开while let块才会被释放，那么就只有一个线程会生效
            loop {
                // 多线程共享的单接收者接收任务，没有在执行任务的线程可以lock住接收到线程
                let job = reciver.lock().unwrap().recv();
                // 获取完任务锁就释放
                match job {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }, // 接收到任务就执行
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        ));
        Self { id, thread }
    }
}

fn main() {
    let pool = ThreadPool::new(5);
    for num in 0..100 {
        pool.execute(move || {
            sleep(Duration::from_secs(3));
            println!("{num}");
        });
    }
}
