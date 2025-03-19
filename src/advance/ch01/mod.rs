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
}
