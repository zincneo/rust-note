/*
# if let 作用域变更
## 相关概念

1. 临时值(temporary value): 类似C++中右值的概念，在表达式求值过程中创建的、没有被绑定到变量的中间值
2. 作用域/生命周期(Scope/Lifetime): 一个值或者引用会在代码中保持有效的区域称为作用域，rust通过所有权原则在作用域结束的时候Drop对应的变量和内存
3. 销毁: Rust中变量离开作用域会自动调用该类型的Drop特征中的drop方法清理资源
4. 被检查者(Scrutinee): 在`if let $pattern = $expression`表达式中，$expression是被检查者，也就是被匹配的表达式
5. 死锁(Deadlock): 并发编程中多个线程对于同一个资源竞争使用导致的如循环等待之类使得所有线程无法继续执行的状态

## if let的问题

- 在过去的标准中`if let`表达式中产生的中间值生命周期可能过长，导致意外的死锁，2024中缩短了临时值的生命周期
*/
fn main() {
    use std::sync::{Arc, RwLock};
    use std::thread::spawn;

    let num = Arc::new(RwLock::new(1));

    let thread = spawn(move || {
        // if let 表达式匹配的num.read()创建了一个读守卫
        if let Ok(_) = num.read() {
        }
        // 新版本编译器在这里就会销毁临时值
        else {
            // 如果进入else分支，代码会尝试创建写守卫
            // 旧版本编译器会造成死锁，因为这里会一直等待if let 匹配的表达式中创建的临时值销毁
            let write = num.write();
            *write.unwrap() += 1;
            println!("不会导致死锁");
        }; // 旧版本编译器会在else块结束才会释放表达式创建的临时值
    });

    let _ = thread.join();
}
