/*!
# Rust函数式编程

Rust中提供了许多函数式语言的特性:

1. 将函数作为参数传递
2. 使用函数作为函数的返回值
3. 将函数赋值给变量

## 闭包

1. [闭包概念和简化代码](./ch14_01_closure/fn.f01_01_closure.html)
2. [闭包类型推导](./ch14_01_closure/fn.f01_02_type.html)
3. [捕获作用域中的值](./ch14_01_closure/fn.f01_03_capture.html)
4. [闭包作为函数的返回值](./ch14_01_closure/fn.f01_04_return.html)
5. 闭包的生命周期
    - 闭包的签名不完整，因此设计到返回引用类型要推导生命周期非常麻烦
    - 解决方式是最好不要在这种情况下使用闭包

## 迭代器

1. 迭代器是什么
2. next方法模拟for循环
3. 两个特征
4. 消费者和适配器
5. 实现特征
*/

pub mod ch14_01_closure;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch14_01() {
        assert_eq!(ch14_01_closure::f01_01_closure(), ());
        assert_eq!(ch14_01_closure::f01_02_type(), ());
        assert_eq!(ch14_01_closure::f01_03_struct(), ());
        assert_eq!(ch14_01_closure::f01_04_return(), ());
    }
}
