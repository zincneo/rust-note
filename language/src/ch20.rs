/*!
# Rust中的unsafe

1. [裸指针](./fn.f01_raw_pointer.html)
2. [unsafe函数和方法](./fn.f02_unsafe_function.html)
3. ffi
    1. Foreign Function Interface
    2. 通过ffi，rust可以直接调用其他语言的代码
4. 访问和修改可变的静态变量
5. 访问union中的字段
    1. union类型只用在和C代码交互，访问union中的字段是不安全的
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
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    ```
4. 创建裸指针并非unsafe行为，使用才是
    ```rs
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("{}", *r1);
        *r2 += 1;
    }
    ```
5. 通过usize数值直接创建裸指针
    ```rs
    let address = 0x012345usize;
    let _r = address as *const i32;
    ```
6. 使用*运算符可以解除裸指针
*/
pub fn f01_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("{}", *r1);
        *r2 += 1;
    }
    println!("{}", num);
    let address = 0x012345usize;
    let _r = address as *const i32;
}

/**
# unsafe方法和函数
1. unsafe方法只需要在fn前面加上unsafe关键字
    1. unsafe函数中不需要使用unsafe块，整体都是unsafe的
2. 只能在unsafe块中调用unsafe方法
3. 要在函数中使用unsafe块并不需要把整个函数变成unsafe块
*/
pub fn f02_unsafe_function() {
    use std::slice;

    fn _split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch20_01() {
        assert_eq!(f01_raw_pointer(), ());
    }

    #[test]
    fn ch20_02() {
        assert_eq!(f02_unsafe_function(), ());
    }
}
