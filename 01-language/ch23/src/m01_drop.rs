#![allow(unused)]
#![allow(dead_code)]

/**
# drop方法
- Rust在编译期就决定好变量应该在什么时候回收和销毁(所有权规则)
- 对于引用变量Rust则使用生命周期概念进行管理(见前面章节)
- 如果希望提前回收一个变量，那么需要使用std::mem::drop方法
    - 该方法针对的是没有Copy特征的对象，因为实现Copy特征的类型将变量作为参数会发送栈上拷贝，提前回收没有作用
*/
pub fn f01_drop_function() {
    {
        let tmp_str = "string".to_string();
        std::mem::drop(tmp_str); // 提前回收该变量
    } // 如果没有提前回收，按照所有权原则，当变量离开对应作用域时，其持有的内存也应该被释放
}

/**
# Drop 特征
- 该特征用来为类型实现在内存释放的时候必要执行的代码(有默认实现)
- 该特征实现的drop方法会在具有所有权的变量离开作用域的时候自动调用
- 调用规则
    1. 同一作用域销毁时，先调用后定义的变量的drop再调用先定义的变量的drop
    2. 组合结构体，先调用外部的drop再调用被包含结构体的drop(按照字段定义的顺序调用)
- Drop特征的drop方法不可以主动调用，要提前结束使用std::mem::drop
*/
pub fn f02_drop_trait() {
    struct A;
    struct B;
    struct C(A, B);
    struct D {
        a: A,
        b: B,
    }
    impl Drop for A {
        fn drop(&mut self) {
            println!("drop A");
        }
    }
    impl Drop for B {
        fn drop(&mut self) {
            println!("drop B");
        }
    }
    impl Drop for C {
        fn drop(&mut self) {
            println!("drop C");
        }
    }
    impl Drop for D {
        fn drop(&mut self) {
            println!("drop D");
        }
    }

    {
        let c = C(A, B);
        let d = D { a: A, b: B };
    }
    // drop D
    // drop A
    // drop B
    // drop C
    // drop A
    // drop B
}
