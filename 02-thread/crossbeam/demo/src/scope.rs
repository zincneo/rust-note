pub fn run() {
    {
        use crossbeam::thread::scope;
        let data = [1, 2, 3, 4, 5];
        // 作用域线程，可以直接借用栈上的值，不需要使用Arc包裹
        let _ = scope(|s| {
            s.spawn(|_| {
                // 直接借用data，不需要通过Arc传递
                let sum: i32 = data[0..2].iter().sum();
                println!("crossbeam scope prev: {sum}");
            });
            s.spawn(|_| {
                let sum: i32 = data[0..5].iter().sum();
                println!("crossbeam scope next: {sum}");
            });
            // 作用域线程会阻塞直到创建的子线程执行完毕
        });
    }

    // 标准库现在也提供了scop
    {
        use std::thread::scope;
        let data = [1, 2, 3, 4, 5];
        // 作用域线程，可以直接借用栈上的值，不需要使用Arc包裹
        let _ = scope(|s| {
            s.spawn(|| {
                // 直接借用data，不需要通过Arc传递
                let sum: i32 = data[0..2].iter().sum();
                println!("std::thread scope prev: {sum}");
            });
            s.spawn(|| {
                let sum: i32 = data[0..5].iter().sum();
                println!("std::thread scope next: {sum}");
            });
            // 作用域线程会阻塞直到创建的子线程执行完毕
        });
    }
}
