#![allow(dead_code)]
#![allow(unused)]

/**
# 迭代器

1. 迭代器就是Rust中遍历一个集合的工具
2. 迭代器本身是实现了迭代器Iterator特征的类型
3. for循环本质上是迭代器的语法糖
    - 能够自动将一个类型转换为迭代器类型并且遍历每一个元素
4. 只要类型实现了IntoIterator特征就可以被转换为迭代器
5. 迭代器本身是惰性的，也就是将一个类型转换成迭代器之后不使用它，不会发生任何事情
```
*/
pub fn f01_iterator() {
    let arr = [1, 2, 3];
    for v in arr.into_iter() {
        println!("{}", v);
    }
    for i in 0..10 {
        println!("{}", i);
    }
}

/**
# next方法模拟for循环
- for循环本身就是迭代器的语法糖
- Iterator特征中具有next方法
- 该方法会返回一个包裹具体元素类型的Option枚举
*/
pub fn f02_next() {
    let arr = [1, 2, 3, 4, 5, 6];
    let mut iter = arr.iter();

    while let Some(v) = iter.next() {
        println!("{v}");
    }
}

/**
# 两个特征
- 迭代器相关的特征
- 实现Iterator特征的类型就是一个迭代器，可以通过next方法获取下一个元素
- 实现IntoIterator特征的类型可以被转换成迭代器
    - into_iter 拿走元素所有权
    - iter 借用
    - iter_mut 可变借用
*/
pub fn f03_trait() {
    let value = vec![1, 2, 3];
    let iter = value.into_iter();
    for v in iter {
        println!("{v}");
    }
}

/**
# 消费者和迭代器
- 迭代器上的方法有两类，称为消费者适配器和迭代器适配器
- 消费者适配器会消耗掉迭代器(依赖next方法)
- 迭代器适配器则不会消耗迭代器而是返回一个迭代器(链式调用的核心)
- 最后要用一个消费者适配器方法来进行收尾，否则什么都不会发生
*/
pub fn f04_adapter() {
    let vec = [1, 2, 3, 4, 5, 6];
    // iter方法将集合类型Vec转换为一个迭代器
    // filter是迭代器适配器，只会转换出一个新的迭代器
    // 最后希望前面迭代器所作的操作产生效果就需要使用消费者适配器进行收尾，调用sum方法
    let sum: i32 = vec.iter().filter(|v| *v % 2 == 0).sum();
    println!("sum: {sum}");
}

/**
# 实现特征
- 迭代器特征中只有next没有默认实现，其它方法均有默认实现
*/
pub fn f05_impl() {
    struct Counter {
        count: i32,
    }
    impl Counter {
        fn new() -> Self {
            Self { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    for i in Counter::new() {
        println!("{i}");
    }
}
