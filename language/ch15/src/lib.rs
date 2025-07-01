#![allow(dead_code)]
#![allow(unused)]

/**
# Box
- 最简单的智能指针，作用是将一个值强制分配在堆上
- 场景的使用场景
    1. 配合特征对象实现多态
    2. 结构体自包含时编译器无法推导出大小，但是又需要固定大小才能通过编译时使用Box包裹
    3. 希望变量绑定以及函数传参的时候不要发生Copy而是发生Move
    4. 使用Box::leak方法主动制造内存泄漏来获取一个全局变量
*/
pub fn f01_box() {
    // 1. 配合特征对象使用
    {
        use std::any::Any;
        trait Draw: Any {
            fn draw(&self) {
                let id = self.type_id();
                println!("{:?}", id);
            }
        }
        struct A;
        struct B;
        impl Draw for A {}
        impl Draw for B {}
        let v: [Box<dyn Draw>; 2] = [Box::new(A), Box::new(B)];
        for e in v {
            e.draw();
        }
    }
    // 2. 自包含结构体/枚举
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(3, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
    // 3. 当栈上数据过大，不想发生Copy的时候可以使用Box强制分配到堆上
    // 注意就算强制分配到堆上也会先在栈上创建，因此如果本身大到爆栈还是会导致栈溢出
    let arr: Box<[i32; 100_000_usize]> = Box::new([0; 100_000_usize]);
    // 4. 主动leak
    fn gen_static_str() -> &'static str {
        let mut s = String::new();
        s.push_str("hello, world");
        Box::leak(s.into_boxed_str())
    }
    let leaked_str = gen_static_str();
    println!("{}", leaked_str);
    // 主动leak的数据可以重建回原来的Box
    unsafe {
        // 获取指针和长度
        let ptr = leaked_str.as_ptr() as *mut u8;
        let len = leaked_str.len();

        // 重建原始 Box
        let reclaimed = Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len));

        // 转换为 String 以证明所有权
        let _string = String::from_utf8(reclaimed.into_vec()).unwrap();
    }
}

/**
# 引用计数Rc
- Rc是标准库提供的引用计数
- 作用是允许一个数据同时被多有所有者持有
- 会导致循环引用问题
- 多线程不安全
*/
pub fn f02_rc() {
    let s = 1;
    let a = Box::new(s); // s所有权转移给a
    // let b = Box::new(s); // 报错

    use std::rc::Rc;
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&b));
}

/**
# 内部可变性Cell和RefCell
- Cell和RefCell提供在变量不使用mut修饰的时候修改内部值的能力
- 通常和Rc结合使用
- Cell适用于实现了Copy特征的类型
    - 提供get和set两个API来访问和设置内部的值
- RefCell适用于没有实现Copy特征的类型
    - 提供了brrow和brrow_mut两个API来获取到可变和不可变引用
    - 主要用来解决可变引用和不可变引用无法共存的问题
    - 其实RefCell也无法同时使用可变引用和不可变引用，只是将编译期的检查换到了运行期的程序panic
*/
pub fn f03_cell_refcell() {
    use std::cell::{Cell, RefCell};
    let num = Cell::new(10);
    num.set(20);
    println!("{}", num.get());

    let s = RefCell::new(String::from("hello, world"));
    let mut ref_s = s.borrow_mut();
    ref_s.push_str("!");
    println!("{:?}", s);

    // Rc + RefCell
    // 单线程其实不怎么使用
    // 多线程下会介绍Arc + Mutex提供多线程安全的引用计数和内部可变性，这就是线程共享的可变数据了
    {
        let s = std::rc::Rc::new(RefCell::new("test string".to_string()));
        let ref_s1 = s.clone();
        let ref_s2 = s.clone();
        ref_s1.borrow_mut().push_str(", end!");
        println!("{:?}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_box() {
        assert_eq!(f01_box(), ());
    }

    #[test]
    fn test02_rc() {
        assert_eq!(f02_rc(), ());
    }

    #[test]
    fn test03_cell_refcell() {
        assert_eq!(f03_cell_refcell(), ());
    }
}
