#![allow(unused)]
#![allow(dead_code)]

use std::ops::Deref;

/**
# 引用参数
- 函数参数使用引用类型的好处
    1. 避免所有权转移或者需要Clone导致性能消耗
    2. 得到自动类型转换能力
- 参数传递时能够自动类型转换其实是触发了参数类型的Deref特征
*/
pub fn f01_ref_arg() {
    fn print_str(s: &str) {
        println!("{s}");
    }

    let s = "string";
    print_str(s);
    let s = "string".to_string();
    print_str(&s);
    let s = &s;
    print_str(s);
}

/**
# Deref特征
- 常规引用是一个指针类型，包含了目标数据存储的内存地址
    - 对常规引用使用 * 操作符，就可以通过解引用的方式获取到内存地址对应的数据值
- 对智能指针或自定义实现了Deref特征的类型使用*运算符，则会触发Deref特征
    - 例如:对于`let x = Box::new(1);`进行`*x`会触发`*(x.deref())`
    - 也就是说由于触发了Deref特征，因此*运算符最终对`deref()`的结果进行解除引用
    - 注意*运算符并不会导致Deref特征连续触发，出现`*(x.deref().deref())`这种情况
- 触发Deref特征的总结
    1. *运算符对于非常规引用类型可以触发一次
    2. 函数参数传递显式使用&运算符时可以隐式地连续触发Deref特征，直到直到合适的类型
- *运算符再次说明
    1. 编译器在处理*运算符的时候只会做一次脱壳处理
    2. 对于`&T`类型转换为`T`类型，对于`&&&&T`同样只处理一个`&`->`&&&T`
    3. 对于`T`智能指针或者实现了Deref特征的自定义类型，那么先调用`T.deref()`->`&U`然后再执行*运算符，得到`U`类型
*/
pub fn f02_deref_trait() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // 对引用类型解引用取到值
    assert_eq!(5, *y);

    // 常规类型多次引用之后使用*运算符的情况
    {
        let y = &y;
        let y = *y; // 只会脱壳一层&
        println!("{:p}, {:p}", y, &x);
    }

    // 智能指针多层包裹使用*运算符的情况
    {
        let y = std::rc::Rc::new(Box::new(x));
        let y = (*y).clone(); // 多层嵌套的情况下*只会触发一次Deref特征，由于是Arc在最外层包裹，因此*的时候触发了Arc的Deref特征最后变成了*(&Box类型)
    }

    // 对比自定义结构体在实现了Deref特征情况下，*运算符和作为实参的区别
    {
        struct MyBox<T>(T);
        impl<T> Deref for MyBox<T> {
            // 指定自动解引用得到的关联类型为类型的泛型参数T
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let s = Box::new(MyBox("custom".to_string()));
        // let t = *s; // 只会触发一次自动解引用

        fn print_str(s: &str) {
            println!("{s}");
        }

        // 显式使用&运算符
        print_str(&s); // 可以连续触发自动解引用
        print_str(&MyBox("custom".to_string()));
    }
}
