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
}
