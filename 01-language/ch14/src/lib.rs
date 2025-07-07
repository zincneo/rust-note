/*!
# 函数式编程

Rust并非纯函数式的语言，但是提供了一些函数式语言中的特性

1. 将函数作为参数传递
2. 使用函数作为函数的返回值
3. 将函数赋值给变量
*/
mod m01_closure;
mod m02_iterator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_closure() {
        assert_eq!(m01_closure::f01_closure(), ());
        assert_eq!(m01_closure::f02_type(), ());
        assert_eq!(m01_closure::f03_capture(), ());
        assert_eq!(m01_closure::f04_return(), ());
        assert_eq!(m01_closure::f05_struct(), ());
    }

    #[test]
    fn test02_iterator() {
        assert_eq!(m02_iterator::f01_iterator(), ());
        assert_eq!(m02_iterator::f02_next(), ());
        assert_eq!(m02_iterator::f03_trait(), ());
        assert_eq!(m02_iterator::f04_adapter(), ());
        assert_eq!(m02_iterator::f05_impl(), ());
    }
}
