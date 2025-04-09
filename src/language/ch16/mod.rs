/*!
# Rust智能指针

## [Box](./fn.f01_box.html)
*/

/**
# Box

1. 最简洁明了的智能指针`Box<T>`
2. `Box<T>`允许分配内存在堆上而非栈上
3. 栈上持有一个指针指向堆上的地址
4. `Box<T>`通常用于以下场景
    1. 编译期无法推导出内存大小，上下文又需要确定大小的类型
    ```rust
    enum List {
        Cons(i32, Box<List>),
        Nil
    }
    ```
    2. 希望在使用变量绑定的时候发生所有权转移而非是Copy
    ```rust
    let array_1 = Box::new([0; 10000]);
    let array_2 = array_1; // transfer ownership
    ```
    3. 特征对象
    ```rust
    trait Draw {
        fn draw(&self);
    }

    let elements = Vec<Box<dyn Draw>>;
    ```
5. Box::leak主动造成内存泄漏，让一个需要在运行期初始化的变量成为全局变量
```rust
fn main() {
   let s = gen_static_str();
   println!("{}", s);
}

fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}
```
*/
pub fn f01_box() {
    let arr_1 = Box::new([0; 10000]);
    let _arr_2 = arr_1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch16_01() {
        assert_eq!(f01_box(), ());
    }
}
