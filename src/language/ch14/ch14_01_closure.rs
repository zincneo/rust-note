/**
# 闭包概念和简化代码

- 闭包是一种匿名函数
- 可以赋值给变量，也可以作为参数传递给其他函数
- 不同于函数，闭包可以捕获**调用者**作用域中的值
- 利用可以捕获环境中值的特性可以简化代码
    - 如果使用传统的函数，每次调用需要反复传入值
    ```rust
    fn action(num: i32) -> i32 {
        println!("{num}");
        num
    }
    fn use_function(num: i32, random_num: i32) {
        if num < 25 {
            println!("first action {}", action(num));
        } else if random_num == 3 {
            println!("second action {}", action(num));
        } else {
            println!("third action {}", action(num));
        }
    }
    ```
    - 使用闭包，自动捕获环境中的值
    ```rust
    fn use_closure(num: i32, random_num: i32) {
        let action = || println!("{num}");
        if num < 25 {
            print!("first action ");
            action();
        } else if random_num == 3 {
            print!("second action ");
            action();
        } else {
            print!("third action ");
            action();
        }
    }
    ```

*/
pub fn f01_01_closure() {
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));

    fn action(num: i32) -> i32 {
        println!("{num}");
        num
    }
    fn use_function(num: i32, random_num: i32) {
        if num < 25 {
            println!("first action {}", action(num));
        } else if random_num == 3 {
            println!("second action {}", action(num));
        } else {
            println!("third action {}", action(num));
        }
    }
    use_function(10, 3);

    fn use_closure(num: i32, random_num: i32) {
        let action = || println!("{num}");
        if num < 25 {
            print!("first action ");
            action();
        } else if random_num == 3 {
            print!("second action ");
            action();
        } else {
            print!("third action ");
            action();
        }
    }
    use_closure(10, 3);
}

/**
# 闭包类型推导
- 函数必须注明参数和返回值的类型
- 闭包则享受编译器提供的类型推导功能
- 完整的声明形式`|arg: type, ...| -> type {}`
- 在编译器能够推导的情况下，可以省略参数的类型和返回值类型
- 要注意的是，闭包并非泛型，编译器第一次推导出来的类型将一直使用
```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```
*/
pub fn f01_02_type() {}

/**
# 捕获作用域中的值
- 闭包可以捕获所在作用域中的值
- 捕获的三种途径对应于函数传参的三种方式:转移所有权、可变借用、不可变借用
- 闭包的类型相关的三个特征FnOnce、FnMut、Fn
    1. FnOnce会拿走闭包的所有权，只能运行一次
        - 这里指的只能运行一次是指这个闭包调用一次就会以self的方式获取到自身的所有权，导致闭包本身的所有权转移不可以再次被调用
        - 看源码可以很明显地看出FnOnce特征包含的方法`call_once`第一个参数是self
    2. FnMut以可变形式获取到环境中的值
        - FnMut特征中的方法`call_mut`第一个参数是`&mut self`
    3. Fn以不可变引用形式获取到环境中的值
- 一个闭包实现哪种Fn特征是取决于闭包如何使用捕获的值，即闭包类型本身的self被如何修饰，而不是闭包如何捕获它们
    ```rust
    let x = 10;
    let test = || {
        println!("{x}");
    };
    fn fn_once<F>(func: F)
    where
        // FnOnce强调的是闭包调用的时候使用self获取到自身而不是捕获方式
        F: FnOnce() -> (),
    {
        func();
        // func不能再次被调用
    }
    fn_once(test);
    ```
- move关键字则强调将捕获变量强制转移所有权进入闭包
    ```rust
    let x = 10;
    let test = move || {
        println!("{}", x);
    };
    fn fn_once<F>(func: F)
    where
        F: FnOnce() -> (),
    {
        func();
    }
    fn_once(test);
    println!("{}", x);
    ```
- 三种特征之间的关系
    - 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
    - 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
    - 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
*/
pub fn f01_03_struct() {
    let x = 10;
    let test = move || {
        println!("{}", x);
    };
    fn fn_once<F>(func: F)
    where
        F: FnOnce() -> (),
    {
        func();
    }
    fn_once(test);
    println!("{}", x);
}

/**
# 闭包作为函数的返回值
- 每一个闭包都是独立类型
- 因此要将闭包作为函数的返回值类型需要使用特征对象
```rust
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
```
*/
pub fn f01_04_return() {}

/**
# 闭包中结构体
- 结构体的字段使用闭包实现缓存的示例
*/
pub fn f01_06_struct() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        query: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(query: T) -> Self {
            Self { query, value: None }
        }
        // 先查询缓存的值，不存在则使用`query`加载
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut cacher = Cacher::new(|arg| arg);
    println!("{}", cacher.value(4));
    println!("{}", cacher.value(20));
}
