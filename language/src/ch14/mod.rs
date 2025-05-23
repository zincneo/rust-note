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
6. [结构体中的闭包](./ch14_01_closure/fn.f01_06_struct.html)

## 迭代器

1. [迭代器是什么](./ch14_02_iterator/fn.f02_01_iterator.html)
2. [next方法模拟for循环](./ch14_02_iterator/fn.f02_02_next.html)
3. [两个特征](./ch14_02_iterator/fn.f02_03_trait.html)
4. [消费者和迭代器](./ch14_02_iterator/fn.f02_04_adapter.html)
5. [实现特征](./ch14_02_iterator/fn.f02_05_impl.html)
*/

pub mod ch14_01_closure;
pub mod ch14_02_iterator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch14_01() {
        assert_eq!(ch14_01_closure::f01_01_closure(), ());
        assert_eq!(ch14_01_closure::f01_02_type(), ());
        assert_eq!(ch14_01_closure::f01_03_struct(), ());
        assert_eq!(ch14_01_closure::f01_04_return(), ());
        assert_eq!(ch14_01_closure::f01_06_struct(), ());
    }

    #[test]
    fn ch14_02() {
        assert_eq!(ch14_02_iterator::f02_01_iterator(), ());
        assert_eq!(ch14_02_iterator::f02_02_next(), ());
        assert_eq!(ch14_02_iterator::f02_03_trait(), ());
        assert_eq!(ch14_02_iterator::f02_04_adapter(), ());
        assert_eq!(ch14_02_iterator::f02_05_impl(), ());
    }
}
