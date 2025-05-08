/*!
# Rust中的unsafe

1. [裸指针](./fn.f01_raw_pointer.html)
*/

/**
# 裸指针
1. 通过`*const T`和`*mut T`来表示原生指针
2. 原生指针的作用
    1. 绕过Rust借用规则，可以同时拥有一个数据的可变和不可变指针，还能拥有多个可变指针
    2. 不能保证安全访问内存
    3. 可以是null
    4. 没有实现任何自动回收
3. 基于引用可以创建裸指针，通过as关键字可以将引用类型转换成对于的原始指针
    ```rs
    let mut num = 5;
    let _r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;
    ```
*/
pub fn f01_raw_pointer() {
    let mut num = 5;
    let _r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;
}
