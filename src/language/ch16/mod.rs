/*!
# Rust智能指针

1. [Box](./fn.f01_box.html)

2. [Deref](./ch16_02_deref/index.html)

3. [Drop](./fn.f03_drop.html)
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

pub mod ch16_02_deref;

/**
# Drop

- Rust中没有GC，通过所有权机制由编译器在编译期就决定变量应该在何时回收
- 可以为类型实现Drop特征，当变量在释放的时候会执行Drop特征中定义的drop方法进行资源回收
    - 变量级别，按照逆序的方式，后定义先先执行drop方法
    - 结构体内部，按照顺序的方式，先定义的字段先执行drop方法
```rust
struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}
{
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
}
// Dropping Foo // 后定义的变量先执行drop方法
// Dropping HasTwoDrop // 嵌套的结构体先执行外部的drop方法
// Dropping HasDrop1 // 结构体内部按照定义的顺序，更早定义的字段先执行drop方法
// Dropping HasDrop2
```
- 禁止手动调用Drop特征中的drop方法，如果需要提前回收要使用std::mem::drop
```rust
let a = Foo;
std::mem::drop(a);
```
- 大部分情况下不需要自定义Drop特征，常见的使用场景
    - 文件描述符、网络socket
    - 一些特殊的数据结构，如没法形成尾递归优化且回收会导致爆栈的结构(例如链表)
*/
#[allow(dead_code)]
pub fn f03_drop() {
    struct HasDrop1;
    struct HasDrop2;
    impl Drop for HasDrop1 {
        fn drop(&mut self) {
            println!("Dropping HasDrop1!");
        }
    }
    impl Drop for HasDrop2 {
        fn drop(&mut self) {
            println!("Dropping HasDrop2!");
        }
    }
    struct HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    }
    impl Drop for HasTwoDrops {
        fn drop(&mut self) {
            println!("Dropping HasTwoDrops!");
        }
    }

    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!")
        }
    }

    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
    println!("Running!");
    {
        let a = Foo;
        std::mem::drop(a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch16_01() {
        assert_eq!(f01_box(), ());
    }

    #[test]
    fn ch16_02() {
        assert_eq!(ch16_02_deref::f02_01_star_mark(), ());
        assert_eq!(ch16_02_deref::f02_02_auto_deref(), ());
        assert_eq!(ch16_02_deref::f02_03_deref_impl(), ());
    }

    #[test]
    fn ch16_03() {
        assert_eq!(f03_drop(), ());
    }
}
