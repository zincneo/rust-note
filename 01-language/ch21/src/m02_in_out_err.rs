#![allow(dead_code)]
#![allow(unused)]

/**
# 标准输出/错误
- io模块提供的方法stdout返回Stdout结构体,stderr返回Stderr结构体
- 输出相关的宏`print!`|`println!`宏也是通过Stdout进行输出，相对应的`eprint!`|`eprintln!`宏通过Stderr进行输出
- 内部为了线程安全会需要上锁，手动锁住可以提高效率
- 一般很少直接使用该方法，正常来说标准输出使用`print!`|`println!`宏即可
*/
pub fn f01_stdout() {
    use std::io::Write;
    let mut stdout = std::io::stdout();
    {
        let handle = stdout.lock();
        stdout.write(&[97, 98, 99]).unwrap();
    }
    let mut stderr = std::io::stderr();
    {
        let handle = stderr.lock();
        stderr.write(&[97, 98, 99]).unwrap();
    }
}

/**
# 标准输入
- io模块提供的方法stdin返回Stdin结构体
- 最常用的是Stdin本身的方法read_line，按行读入到字符串中
- 除此之外导入Read特征，可以使用Read特征上的读取方法
- 内部为了线程安全会需要上锁，手动锁住可以提高效率
*/
pub fn f02_stdin() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("You typed: {}", input.trim());
}
