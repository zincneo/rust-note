/*!
# Rust智能指针

## [1. Box](./fn.f01_box.html)

## [2. Deref](./ch16_02_deref/index.html)

## [3. Drop](./fn.f03_drop.html)

## [4. Rc和Arc](./ch16_04_rc_arc/index.html)

## [5. Cell和RefCell](./ch16_05_cell_refcell/index.html)

## [6. Weak](./fn.f06_weak.html)
*/

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

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

pub mod ch16_04_rc_arc;

pub mod ch16_05_cell_refcell;

/**
# Weak

- Weak智能指针用来解决Rc导致的循环引用问题
- 需要使用Rc::downgrade关联方法来获取
- Rc和Weak的对比

| **Rc** | **Weak** |
|:---:|:---:|
| 不计数 | 引用计数 |
| 不拥有所有权 | 拥有值的所有权 |
| 不阻止值drop | 所有引用计数归0才会drop |
| 引用存在返回Some,不存在返回None | 引用必然存在 |
| 通过 upgrade 取到 Option<Rc<T>>，然后再取值 | 通过Deref触发自动解引用，无需额外操作 |

- 链表使用循环引用问题
    ```rust
    #[derive(Debug)]
    enum List {
        Cons(char, RefCell<Rc<List>>),
        Nil,
    }
    use List::*;
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
    let a = Rc::new(Cons('A', RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons('B', RefCell::new(Rc::clone(&a))));
    // 制造A -> B, B -> A
    if let Some(item) = a.tail() {
        *item.borrow_mut() = Rc::clone(&b);
    }
    // 导致爆栈
    println!("a next item = {:?}", a.tail());
    // 在主进程结束之前，a和b的引用计数都为2
    // b先触发drop，这时候b引用计数-1，b的引用计数为1
    // a后触发drop，这时候a引用计数-1，a的引用计数为1
    // 这样导致a，b所指向的堆上内存都始终有1引用计数，无法被释放，造成了内存泄漏
    ```
    - 使用Weak解决循环引用问题
    ```rust

    #[derive(Debug)]
    enum List {
        Cons(char, RefCell<Weak<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Weak<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    use List::*;
    let nil = Rc::new(Nil);
    let a = Rc::new(Cons('A', RefCell::new(Rc::downgrade(&nil))));
    let b = Rc::new(Cons('B', RefCell::new(Rc::downgrade(&a))));
    if let Some(item) = a.tail() {
        *item.borrow_mut() = Rc::downgrade(&b);
    }
    println!("{:#?}", a);
    // a, b释放的时候引用计数-1，引用计数为0，堆上数据正确释放
    ```
*/
#[allow(dead_code)]
pub fn f06_weak() {
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);
    std::mem::drop(five);
    let strong_five = weak_five.upgrade();
    assert_eq!(strong_five, None);

    #[derive(Debug)]
    enum List {
        Cons(char, RefCell<Weak<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Weak<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    use List::*;
    let nil = Rc::new(Nil);
    let a = Rc::new(Cons('A', RefCell::new(Rc::downgrade(&nil))));
    let b = Rc::new(Cons('B', RefCell::new(Rc::downgrade(&a))));
    if let Some(item) = a.tail() {
        *item.borrow_mut() = Rc::downgrade(&b);
    }
    println!("{:#?}", a);
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

    #[test]
    fn ch16_04() {
        assert_eq!(ch16_04_rc_arc::f04_01_rc(), ());
        assert_eq!(ch16_04_rc_arc::f04_02_case(), ());
    }

    #[test]
    fn ch16_05() {
        assert_eq!(ch16_05_cell_refcell::f05_01_cell(), ());
        assert_eq!(ch16_05_cell_refcell::f05_02_refcell(), ());
        assert_eq!(ch16_05_cell_refcell::f05_03_rc_refcell(), ());
    }

    #[test]
    fn ch16_06() {
        assert_eq!(f06_weak(), ());
    }
}
