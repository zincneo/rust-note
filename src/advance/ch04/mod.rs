/**
## 循环引用问题
- Rc和RefCell结合使用的时候可以制造出来循环引用
- 引用计数无法被归零，最后导致panic
*/
fn _ch04_01_circular_referance() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    use List::{Cons, Nil};
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    println!("a指向的节点 = {:?}", a.tail());
    // 创建`b`->`a`的引用
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    // 当前链表 b[10, ->a] -> a[5, nil]
    println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    println!("b指向的节点 = {:?}", b.tail());
    // 现在利用内部可变性RefCell修改a的下一个节点
    // 利用RefCell的可变性，创建了`a`->`b`的引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    // 当前链表 b[10, ->a] -> a[5, ->b]
    println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));
    // 尝试输出循环引用就会到导致栈空间溢出
    // println!("a next item = {:?}", a.tail());
}

/**
## Weak智能指针
- 非常类似Rc，但是不持有数据的所有权
- 和Rc的对比
  - Weak(不计数) Rc(计数)
  - Weak(不拥有堆上数据所有权) Rc(拥有数据所有权)
  - Weak(不能阻止值被drop) Rc(引用的值必定存在，计数清零才会drop)
  - Weak(通过upgrade得到Option<Rc<T>>然后再使用) Rc(直接触发Deref使用)
*/
fn _ch04_02_weak() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    // 1. weak使用
    {
        let five = Rc::new(5);
        // 从Rc降级到Weak
        let weak_five = Rc::downgrade(&five);
        let strong_five = weak_five.upgrade();
        if let Some(value) = strong_five {
            assert_eq!(*value, 5);
        };
        // 释放five
        drop(five);
        let strong_five = weak_five.upgrade();
        assert_eq!(strong_five, None);
    }
    // 2. 替换掉Rc，解决循环引用问题
    {
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Weak<List>>),
            Nil,
        }
        use List::{Cons, Nil};
        impl List {
            fn tail(&self) -> Option<&RefCell<Weak<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }
        let nil = Rc::new(Nil);
        let a = Rc::new(Cons(5, RefCell::new(Rc::downgrade(&nil))));
        let b = Rc::new(Cons(5, RefCell::new(Rc::downgrade(&a))));
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::downgrade(&b);
        };
        println!("a next item = {:?}", a.tail());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch04_01() {
        assert_eq!(_ch04_01_circular_referance(), ());
    }

    #[test]
    fn ch04_02() {
        assert_eq!(_ch04_02_weak(), ());
    }
}
