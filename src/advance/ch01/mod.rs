/**
## 闭包
- 在rust中closure指匿名函数
- 闭包可以赋值给变量也可以作为参数传递给其他函数
- 允许捕获调用者作用域中的值
- 闭包的定义形式`|param_list| return_value_expression`
    - 参数列表用法和函数相同
    - 只有一个返回值表达式的时候可以不加花括号
    - 有多个语句的话同样需要`{}`包裹代码块
*/
fn _ch01_01_closure() {
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));

    // 使用闭包简化代码
    // 1. 传统函数实现
    {
        use std::thread;
        use std::time::Duration;
        fn muuuu(intensity: u32) -> u32 {
            println!("muuuu....");
            thread::sleep(Duration::from_secs(2));
            intensity
        }
        fn workout(intensity: u32, random_number: u32) {
            if intensity < 25 {
                println!("先做 {} 组俯卧撑!", muuuu(intensity));
            } else if random_number == 3 {
                println!("昨天练过度了，今天还是休息下吧！");
            } else {
                println!("跑步 {} 分钟！", muuuu(intensity));
            }
        }
        let intensity = 10;
        let random_number = 7;
        workout(intensity, random_number);
    }
    // 函数存在的缺陷:如果之后要替换掉muuuu函数每一处都需要修改
    // 解决方式，如果一个函数调用多次，可以将这个函数赋值给一个变量然后使用这个变量，rust允许这种行为

    // 2. 使用闭包实现
    {
        use std::thread;
        use std::time::Duration;
        fn workout(intensity: u32, random_number: u32) {
            let action = || {
                println!("muuuu....");
                thread::sleep(Duration::from_secs(2));
                intensity
            };
            if intensity < 25 {
                println!("先做 {} 组俯卧撑!", action());
            } else if random_number == 3 {
                println!("昨天练过度了，今天还是休息下吧！");
            } else {
                println!("跑步 {} 分钟！", action());
            }
        }
        let intensity = 10;
        let random_number = 7;
        workout(intensity, random_number);
    }
}

/**
## 闭包类型推导
- 由于闭包不对外提供API，因此闭包可以享受类型推导
- 为了增加代码可读性，也可以手动标准类型
- 当编译器推导出一种类型之后，这个闭包将一直使用该类型，并非泛型
*/
fn _ch01_02_type_derivation() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    println!("{}", add_one_v1(1));
    println!("{}", add_one_v2(1));
    println!("{}", add_one_v3(1));
}

/**
## 结构体中的闭包
- 结构体的成员属性类型可以是闭包
- 闭包类型使用特征约束来进行表示
- 每一个闭包都有自己的类型即使是定义完全相同类型也不一样
- 成员变量类型表示需要通过泛型参数和where约束
*/
fn _ch01_03_closure_in_struct() {
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
            Cacher { query, value: None }
        }
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
    let mut cacher = Cacher::new(|x| x + 1);
    let value = cacher.value(10);
    println!("{}", value);
}

/**
## 闭包捕获值
- 闭包可以捕获所在作用中的值
- 闭包捕获值的时候需要分配内存去存储这些值
*/
fn _ch01_04_capture() {
    let x = 4;
    /*
     * 函数无法捕获环境中的值
     * fn equal_to_x(z: i32) -> bool {
     *    z == x
     * }
     */
    let equal_to_x = |z| x == z;
    let y = 4;
    assert!(equal_to_x(y));
}

/**
## 三种闭包特征
- 闭包捕获变量的三种方式对应函数传入参数的三种形式
- 所有权转移 FnOnce
  - 拿走环境中值的所有权，因此只能运行一次
  - 如果在函数体内没有获取外部变量的所有权也只能运行一次，但是可以在条件约束的时候加上Copy特征让其可以多次调用(每次调用的都是新拷贝的版本)
  - 强制转移所有权可以在闭包参数列表前面加上move关键字，这种方式用于闭包生命周期大于捕获变量生命周期的情况，比如将闭包返回或移入其他线程
- 可变借用 FnMut
  - 声明可变闭包的时候变量需要加上mut关键字
- 不可变借用 Fn
- 这三个特征强调的是在闭包内部如何使用捕获的值，而不是关心闭包如何捕获
*/
fn _ch01_05_trait() {
    // 1. FnOnce
    // FnOnce不代表一定转移所有权，而是表示该闭包只能调用一次
    {
        fn fn_once_v1<F>(func: F)
        where
            F: FnOnce(usize) -> bool,
        {
            println!("{}", func(3));
            // 报错 println!("{}", func(4));
        }

        fn fn_once_v2<F>(func: F)
        where
            // 闭包要自动实现Copy特征的条件是被捕获的值都实现了Copy特征
            F: FnOnce(usize) -> bool + Copy,
        {
            println!("{}", func(3));
            // 再次调用的时候调用的是第一个的Copy版本
            println!("{}", func(4));
        }
        let x = vec![1, 2, 3];
        fn_once_v1(|z| z == x.len());
        fn_once_v2(|z| z == x.len());
    }
    // 2. FnMut
    {
        let mut s = String::new();

        // 报错，需要变量本身也是可变的 let update_string = |str| s.push_str(str);
        let mut update_string = |str| s.push_str(str);
        // 不加mut关键字编译器也可以推导出来是FnMut特征约束的类型，但是会导致不允许内部修改捕获的值，因此必须要加上mut关键字
        update_string("hello");

        println!("{:?}", s);
    }
    // 3. Fn
    {
        let s = "hello, ".to_string();

        let update_string = |str| println!("{},{}", s, str);
        update_string("world".to_string())
    }
}

/**
## 三种特征之间的关系
- move表示强制移动所有权进入闭包
- 闭包实现哪种特征和捕获的时候是不是发生所有权转移没有关系，而是取决于闭包内部如何使用这些被捕获的变量
- 闭包实现Fn的规则
    - 一个闭包一定实现FnOnce，因为至少可以被调用一次
    - 对没有移出捕获变量所有权的闭包自动实现FnMut特征
    - 不需要对捕获变量进行修改的变量自动实现Fn特征
*/
fn _ch01_06_fn() {
    {
        fn exec<F: FnMut()>(mut f: F) {
            f()
        }
        let s = String::new();

        // 这里的move强调的是将s移动进入闭包
        // 但是对于闭包实现特征来说，闭包内部没有修改s的值，因此也会自动实现Fn，没有移出捕获值的所有权因此也会自动实现FnMut
        // 因此对于这个闭包来说实现的特征是Fn + FnMut + FnOnce
        let update_string = move || println!("{}", s);

        exec(update_string); // 这里不可以继续使用s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch01_01() {
        assert_eq!(_ch01_01_closure(), ());
    }

    #[test]
    fn ch01_02() {
        assert_eq!(_ch01_02_type_derivation(), ());
    }

    #[test]
    fn ch01_03() {
        assert_eq!(_ch01_03_closure_in_struct(), ());
    }

    #[test]
    fn ch01_04() {
        assert_eq!(_ch01_04_capture(), ());
    }

    #[test]
    fn ch01_05() {
        assert_eq!(_ch01_05_trait(), ());
    }

    #[test]
    fn ch01_06() {
        assert_eq!(_ch01_06_fn(), ());
    }
}
