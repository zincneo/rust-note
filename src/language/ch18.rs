/*!
# 全局变量
- 全局变量的生命周期肯定是'static
- 但不一定要通过static关键字声明
- 常量、字符串字面值等无需使用static进行声明，原因是它们已经被打包到二进制可执行文件中

## 1. 编译期初始化

1. 静态常量
    - 必须使用const关键字而不是let
    - 类型必须注明
    - 全局的静态常量可以在程序的任意一部分使用，在模块中的时候需要引入模块
    - 必须指明类型否则无法通过编译
    - 命名规则一般为全大写
    - 常量可以在任意作用域进行定义，其生命周期贯穿整个程序的生命周期。编译时编译器会尽可能将其内联到代码中，所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址
    - 常量的赋值只能是常量表达式/数学表达式，也就是说必须是在编译期就能计算出的值，如果需要在运行时才能得出结果的值比如函数，则不能赋值给常量表达式
    - 对于变量出现重复的定义(绑定)会发生变量遮盖，后面定义的变量会遮住前面定义的变量，常量则不允许出现重复的定义
    ```rust
    const MAX_ID: usize =  usize::MAX / 2;
    ```
2. 静态变量
    - 必须使用static关键字定义
    - 可以被mut修饰，但是修改必须在unsafe块中(因为在多线程环境下这么干会产生脏数据)
    - 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
    - 存储在静态变量中的值必须要实现 Sync trait
    ```rust
    static mut REQUEST_RECV: usize = 0;
    fn main() {
       unsafe {
            REQUEST_RECV += 1;
            assert_eq!(REQUEST_RECV, 1);
       }
    }
    ```
3. 原子类型
    - 结合static关键字使用，原子类型有内部可变性而且可以安全地在多线程环境下使用
    - 示例见上一节原子操作

## 2. 运行期初始化
1. static关键字声明的变量没办法使用函数初始化，因此如果想要初始化如全局的Mutex之类的数据，就要使用一些其他办法
2. lazy_static
    - 第三方库提供的宏，用于懒初始化静态变量
    ```rust
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref NAMES: Mutex<String> = Mutex::new(String::from("Hello Rust"));
    }
    fn main() {
        let mut v = NAMES.lock().unwrap();
        v.push_str("!");
        println!("{}", v);
    }
    ```
3. Box::leak
    - 通过主动制造内存泄漏来将局部变量赋值给全局变量
    ```rust
    static mut STR: Option<String> = None;

    fn main() {
        unsafe {
            // STR = Some("test".to_string()); // 报错，不可以将局部生命周期的变量赋值给'static生命周期的变量
            STR = Some(Box::leak("test".to_string())); // 通过Box::lead主动制造内存泄漏
        }
    }

    ```
*/
