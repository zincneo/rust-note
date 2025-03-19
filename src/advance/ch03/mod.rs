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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch03_01() {
        assert_eq!(_ch03_01_box(), ());
    }
}
