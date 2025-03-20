/**
## 循环引用问题
- Rc和RefCell结合使用的时候可以制造出来循环引用
- 引用计数无法被归零，最后导致panic
*/
#[allow(unused)]
#[allow(dead_code)]
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
#[allow(unused)]
#[allow(dead_code)]
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

/**
## 自引用问题
- 自定义结构体中字段引用另外一个字段称为自引用
- 所有权引起的问题，定义字段的时候原来变量的所有权已经转移了
- 使用Option枚举
  - 一定程度上可以工作
  - 没法从函数中创建并返回
  - 会凭空创造生命周期
- 使用unsafe无视借用规则
  - 不使用引用而是直接使用裸指针则不会受到所有权的约束，但是需要在unsafe块中使用
- 使用Pin
  - 这个类型可以阻止值的所有权被转移
*/
#[allow(dead_code)]
#[allow(unused)]
fn _ch04_03_self_ref() {
    // 1. 所有权引起的问题
    {
        struct SelfRef<'a> {
            value: String,
            // 该引用指向上面的value
            pointer_to_value: &'a str,
        }
        let s = "aaa".to_string();
        /*
        let v = SelfRef {
            value: s,
            pointer_to_value: &s, // 编译错误，在转移所有权之后使用s
        };
        */
    }
    // 2. 使用Option枚举
    {
        #[derive(Debug)]
        struct WhatAboutThis<'a> {
            name: String,
            nickname: Option<&'a str>,
        }
        let mut tricky = WhatAboutThis {
            name: "Annabelle".to_string(),
            nickname: None,
        };
        tricky.nickname = Some(&tricky.name[..4]);
        println!("{:?}", tricky);
        // 由于生命周期问题，这样就无法从函数中返回这种类型的对象
        // 很明显对于函数来说，'a属于从函数内部凭空生出来的生命周期
    }
    // 3. unsafe
    {
        #[derive(Debug)]
        struct SelfRef {
            value: String,
            pointer_to_value: *const String,
        }
        impl SelfRef {
            fn new(txt: &str) -> Self {
                Self {
                    value: String::from(txt),
                    pointer_to_value: std::ptr::null(),
                }
            }
            fn init(&mut self) {
                let self_ref: *const String = &self.value;
                self.pointer_to_value = self_ref;
            }
            fn value(&self) -> &str {
                &self.value
            }
            fn pointer_to_value(&self) -> &String {
                assert!(
                    !self.pointer_to_value.is_null(),
                    "Test::b called without Test::init being called first"
                );
                unsafe { &*(self.pointer_to_value) }
            }
        }
        let mut t = SelfRef::new("hello");
        t.init();
        // 打印值和指针地址
        println!("{}, {:p}", t.value(), t.pointer_to_value());
    }
    // 4. Pin
    {
        use std::marker::PhantomPinned;
        use std::pin::Pin;
        use std::ptr::NonNull;

        // 下面是一个自引用数据结构体，因为 slice 字段是一个指针，指向了 data 字段
        // 我们无法使用普通引用来实现，因为违背了 Rust 的编译规则
        // 因此，这里我们使用了一个裸指针，通过 NonNull 来确保它不会为 null
        struct Unmovable {
            data: String,
            slice: NonNull<String>,
            _pin: PhantomPinned,
        }

        impl Unmovable {
            // 为了确保函数返回时数据的所有权不会被转移，我们将它放在堆上，唯一的访问方式就是通过指针
            fn new(data: String) -> Pin<Box<Self>> {
                let res = Unmovable {
                    data,
                    // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
                    slice: NonNull::dangling(),
                    _pin: PhantomPinned,
                };
                let mut boxed = Box::pin(res);

                let slice = NonNull::from(&boxed.data);
                // 这里其实安全的，因为修改一个字段不会转移整个结构体的所有权
                unsafe {
                    let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
                    Pin::get_unchecked_mut(mut_ref).slice = slice;
                }
                boxed
            }
        }
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

    #[test]
    fn ch04_03() {
        assert_eq!(_ch04_03_self_ref(), ());
    }
}
