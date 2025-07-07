#![allow(dead_code)]

/**
# 函数
- Rust中使用关键字fn定义函数
- Rust函数命名规则与变量名相同
- 函数的参数列表必须注明每一个参数的类型
- 函数的返回值类型在参数列表后使用`-> type`表示，缺省表示返回空元组
*/
pub fn f01_function() {
    fn _func(a: i32, b: i32) -> i32 {
        a + b
    }
}

/**
# 发散函数
- 在数学分析中，级数分为发散和收敛
    - 1 + 2 + 3 + 4 + ... 或者 1 + (-1) + 1 + (-1) + 1 + (-1) + ... 当n趋近于无穷大的时候，值不会趋近于固定值，这种级数称为发散极数
- 在编程语言中，一个函数调用注定无法结束回到调用它的作用域继续执行，那么这个函数称为发散函数
- Rust在表示一个函数发散的时候返回值使用`-> !`来表示
*/
pub fn f02_divergence_function() {
    // 1. 死循环
    fn forever() -> ! {
        loop {}
    }

    // 2. 会导致线程崩溃的函数
    fn dead_end() -> ! {
        panic!("panic");
    }
}

/**
# 主函数
- Rust编译的二进制可执行程序提供main函数作为程序的入口
- main本身是一个泛型函数，返回值类型是一个特征对象`impl std::process::Termination`即可
    - 满足该特征的类型如()和Result<(),E>是最常见的形式
- main没有参数列表，想获取参数都使用`std::env`进行解决
*/
pub fn f03_main_function() {}

/**
# const函数
- const关键字修饰函数使得该函数会在编译期执行
- 所有调用该函数的地方将会在编译期直接执行并替换为返回值
- 使用场景
    1. 用于计算常量表达式
    2. 结合const泛型使用，见泛型章节
*/
pub fn f04_const_function() {
    const fn cube(num: usize) -> usize {
        num * num * num
    }
    const DIM: usize = cube(2);
    const ARR: [i32; DIM] = [0; DIM];
    println!("{:?}", ARR);
}
