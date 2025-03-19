/**
## Box
- 堆栈概念复习
  - 栈空间从内存地址高位向下分配，连续且大小有限，Rust中main线程大小为8MB，普通线程是2MB
  - 堆空间从内存地址低位向上增长，只受物理大小限制
- Rust中最常见的指针就是引用&类型，但是rust中引用也包含了对数据借用的概念，因此也被叫做借用
- Box智能指针，Rust中最简单的一种智能指针
  - 特意将数据分配在堆上的时候使用
  - 数据比较大的时候，不想在转移所有权的时候发生Copy行为
  - 类型大小在编译期无法确定，但编码时有需要固定大小的类型时
  - 特征对象，用来说明对象实现了某种特征，而非固定类型
- Box本身是栈上的数据结构，内部保存了指向堆上对象地址的指针
- Box::leak这个关联函数可以手动让一个值从内存中泄漏，场景是让运行期初始化的值可以活得和程序一样久
*/
#[allow(unused)]
fn _ch03_01_box() {
    // 1. 使用Box将内存分配在堆上
    let _a = Box::new(3); // Box本身存在栈上，指向的i32分配在堆上

    // 2. 避免栈上数据发生copy
    let _a = [0; 1000];
    let _b = _a; // 将发生copy
    let _a = Box::new([0; 1000]);
    let _b = _a;
    // println!("{:?}", _a); 发生了move行为，_a不再可以被使用
    println!("{:?}", _b);

    // 3. 将动态长度的数据类型变成固定长度
    // 典型的就是自引用类型
    enum List {
        // Cons(i32, List), // 编译器算不出来自引用要占用多大内存
        Cons(i32, Box<List>), // 利用Box让这个类型占据的内存可以被确定
        Nil,
    }

    // 4. 特征对象(就是多态) -> 其他面向对象的语言比如c++通过虚函数表和继承实现多态
    // 所谓多态其实就是不同的数据类型可以调用相同的接口
    trait Draw {
        fn draw(&self) {
            println!("default draw");
        }
    }
    struct Button {
        id: u32,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("This is {} Button", self.id);
        }
    }
    struct Select {
        id: u32,
    }
    impl Draw for Select {
        fn draw(&self) {
            println!("This is {} Select", self.id);
        }
    }
    let elements: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 2 }), Box::new(Select { id: 1 })];
    for element in elements {
        element.draw();
    }

    // 5. Box解引用
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    // 表达式不会隐式解引用，因此要使用**，第一个*从&Box<i32>找到Box<i32>，第二个再从Box<i32>找到i32
    let sum = **first + **second;
}

/**
## Deref
- 通过*运算符可以获取引用的值
- 表达式必须手动调用*解除引用
- 通过实现Deref特征自定义类型可以实现*解引用一样的行为
  - 该特征必须实现方法deref，这个方法会返回一个&引用类型
  - 因此对于实现该特征的结构体调用`*varibale`等价于调用了`*(variable.deref())`
- 函数和方法以及赋值操作会有隐式的解引用行为
  - 若一个类型实现了 Deref 特征，那它的引用在传给函数或方法时，会根据参数签名来决定是否进行隐式的 Deref 转换
  - 注意一定是在实参传递的时候使用&传递才会触发隐式的Deref
  - Deref可以连续触发，直到找到合适的形式
- 引用归一化
  - rust在做解引用的时候会将智能指针和&&&&&v的形式做引用归一化操作转换为&v，再做解引用
- Deref特征有三种情况
  - Deref 不可变/可变 -> 不可变
  - DerefMut 可变 -> 可变
*/
fn _ch03_02_deref() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);

    // 1. 使用*解除&类型的引用
    assert_eq!(5, *y);

    // 2. 使用*解除Box智能指针的引用
    let x = Box::new(5);
    assert_eq!(5, *x);

    // 3. 自定义类型实现Deref特征后也可以使用*运算符解引用
    use std::ops::Deref;
    #[derive(Debug)]
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> Self {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let x = MyBox::new(10);
    println!("{:?}", x);
    let x = *x + 1;
    println!("{:?}", x);

    // 4. 隐式转换
    {
        fn display(s: &str) {
            println!("{}", s);
        }
        let s = String::from("hello world");
        // 1. String类型实现了Deref
        // 2. 调用display的时候传递&String
        // 3. 隐式地调用String的deref方法转换为&str
        // 4. 必须是通过&s这种方式才会触发Deref
        display(&s);
    }

    // 5. 连续触发Deref的例子
    let s = MyBox::new(String::from("hello, world"));
    let _s1: &str = &s; // &s -> &MyBox::deref -> &String::deref -> &str
    let _s2: String = s.to_string();

    // 6. 引用归一化
    // 标准库源代码
    /*
     * impl<T: ?Sized> Deref for &T {
     *   type Target = T;
     *   fn deref(&self) -> &T {
     *     *self
     *   }
     * }
     */
    // 由源码可以看到遇到&&&&v这的情况就会发生递归解引用
    // &(&&&v.deref()) -> &(&&v.deref()) -> &(&v.deref()) -> &(v.deref())
}

/**
## Drop特征
- 如果一个类型实现了Drop特征，则其变量会在离开其作用域的时候自动调用drop方法进行收尾
- 离开一块作用域的时候调用drop的顺序如下
  - 变量级别，按照逆序的方式，后定义的被先调用drop
  - 结构体内的字段，按照字段定义的顺序drop
- 不可以直接调用drop方法，但是Rust提供了std::mem::drop函数来手动提前丢弃一个变量(发生move并且调用变量的drop方法)
- 由于rust当中根本没有new关键字，因此绝大部分时候不需要手动回收，使用场景是需要手动回收资源的比如文件描述符，网络socket这些需要自己实现Drop
- Drop特征和Copy特征互斥
*/
#[allow(dead_code)]
#[allow(unused)]
fn _ch03_03_drop() {
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
    std::mem::drop(_x);
    println!("Running!");
}

/**
## 引用计数
- Rust所有权机制要求一个值只能有一个所有者，但有些场景任然需要一份数据被多个变量持有
  - 图数据结构中，多个边可能会拥有同一个节点，直到没有任何边指向的时候才应该被清理
  - 多线程中，多个线程可能持有同一个数据，但是受限与Rust所有权机制，无法获取多份该数据的可变引用
- Rc 引用计数，不支持多线程安全
  - Rc::new()将包裹的值move到堆上，栈上使用这个智能指针
  - Rc::clone()拷贝一份栈上的智能指针，指向同一块堆内存，且引用计数加1
  - Rc<T>的底层是指向T的不可变引用，因此无法通过Rc来修改数据，如果需要修改值就要用到内部可变性的RefCell包裹T，再让Rc包裹RefCell
  - 如果在创建线程的时候在传递给线程的闭包前加上move关键字，那么在闭包内将Rc传递过去的时候编译器就会直接报错
- Arc 线程安全的引用计数
  - 用法和Rc一样，底层都是不可变引用，但是Arc是保证线程间安全的，可以在不同线程间传递
  - 性能相对与Rc因为要保证原子化因此更差
*/
fn _ch03_04_rc_arc() {
    // 1. 由于所有权导致报错的例子
    {
        let s = String::from("Rust String");
        let _a = Box::new(s); // s发生了所有权的move
                              // let b = Box::new(s); // 报错，因为s移动到了堆上且被a拥有，不可以再使用s
    }
    // 2. 使用Rc引用计数让一个堆上数据被多个所有者拥有
    {
        use std::rc::Rc;
        let a = Rc::new(String::from("hello, world"));
        let b = Rc::clone(&a);
        // 查看引用计数
        assert_eq!(2, Rc::strong_count(&a));
        assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

        let a = Rc::new(String::from("test ref counting"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    // 3. Rc的一个实际的例子
    {
        use std::rc::Rc;
        #[derive(Debug)]
        struct Owner {
            name: String,
        }

        impl Owner {
            fn new(name: String) -> Self {
                Self { name }
            }
        }

        #[derive(Debug)]
        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
        }
        let gadget_owner: Rc<Owner> = Rc::new(Owner::new("Gadget Man".to_string()));

        // 创建两个不同的工具，它们属于同一个主人
        let gadget1 = Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        };
        let gadget2 = Gadget {
            id: 2,
            owner: Rc::clone(&gadget_owner),
        };

        // 释放掉第一个 `Rc<Owner>`
        drop(gadget_owner);
        // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
        // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
        // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
        // 引用指向底层的 owner 数据，引用计数尚未清零
        // 因此 owner 数据依然可以被使用
        println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
        println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

        // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
        // 数据也被清理释放
    }

    // 4. 线程间安全的Arc
    {
        use std::sync::Arc;
        use std::thread;
        let s = Arc::new(String::from("use between multiple threads"));
        for i in 0..10 {
            let s = Arc::clone(&s);
            let _ = thread::spawn(move || {
                println!("{} : {:#?}", i, s);
            })
            .join();
        }
    }
}

/**
## 内部可变性
- Cell和RefCell两个类型提供了在使用不可变引用的情况下修改内部值的能力
- 底层是通过unsafe包裹的
- Cell适用于实现了Copy特征的类型
- Rust本身的所有权规则
  - 一个数据只能有一个所有者
  - 要么是多个不可变借用要么一个可变借用
  - 违反规则编译不通过
- 由于智能指针带来的额外规则
  - Rc/Arc让一个数据拥有多个所有者
  - RefCell实现编译期可变，不可变引用共存
  - 违背规则会导致运行期panic，也就是还是要遵循所有权原则，否则也是绕过编译期到运行的时候panic
- Cell用于提供值，而RefCell用于提供引用
- Cell不会引起panic，而RefCell会
*/
fn _ch03_05_cell_refcell() {
    // 1. Cell
    {
        use std::cell::Cell;
        let c = Cell::new(10);
        let one = c.get();
        c.set(20);
        let two = c.get();
        println!("{} {}", one, two);
    }
    // 2. RefCell
    {
        use std::cell::RefCell;
        {
            let s = RefCell::new(String::from("hello, world"));
            let _s1 = s.borrow();
            // let _s2 = s.borrow_mut(); // 运行期间panic

            // println!("{},{}", s1, s2); // 运行期间panic
        }
    }
    // 3. Rc和RefCell结合使用
    {
        use std::cell::RefCell;
        use std::rc::Rc;
        let s = Rc::new(RefCell::new("chatgpt".to_string()));
        let s1 = s.clone();
        let s2 = s.clone();
        s2.borrow_mut().push_str(" claude");
        println!("{:?} {:?}", s1, s2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch03_01() {
        assert_eq!(_ch03_01_box(), ());
    }

    #[test]
    fn ch03_02() {
        assert_eq!(_ch03_02_deref(), ());
    }

    #[test]
    fn ch03_03() {
        assert_eq!(_ch03_03_drop(), ());
    }

    #[test]
    fn ch03_04() {
        assert_eq!(_ch03_04_rc_arc(), ());
    }

    #[test]
    fn ch03_05() {
        assert_eq!(_ch03_05_cell_refcell(), ());
    }
}
