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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch01_01() {
        assert_eq!(_ch01_01_closure(), ());
    }
}
