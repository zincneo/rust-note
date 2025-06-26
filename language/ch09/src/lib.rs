/*!
# 生命周期
- 所有程序中对于变量都有从在内存中创建到销毁的过程，可以称该段在内存中存在的时间为变量的存活期
- 以上所谓存活期是是运行时的概念
- 由于Rust编译器的强大，因此Rust会在编译期检查一个引用变量可以使用的时间保证不会出现悬垂指针这样的状况
- Rust中生命周期就是指引用变量可以使用的有效作用域

## 生命周期的作用

- 生命周期的主要作用是防止悬垂引用
- 也就是当一个变量被销毁之后其引用就不应该存在
- Rust的生命周期会在编译期做该检查

## 生命周期标识符

- 生命周期标识符是一种标注语法，在引用变量的类型标识符`&`之后添加
- 生命周期标识符只能用于函数、结构体、枚举、方法
- 语法形式是以`'`开头然后通常跟上一个小写字母
- 常见的形式`&'a i32`，`&'b i64`
- 注意生命周期标识符在使用之前都需要在泛型参数列表中声明
*/
mod m01_lifetime;
mod m02_method;
mod m03_struct;
mod m04_static;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_lifetime() {
        assert_eq!(m01_lifetime::f01_lifetime(), ());
        assert_eq!(m01_lifetime::f02_function(), ());
    }

    #[test]
    fn test02_method() {
        assert_eq!(m02_method::f01_method(), ());
    }

    #[test]
    fn test03_struct() {
        assert_eq!(m03_struct::f01_struct(), ());
    }

    #[test]
    fn test04_static() {
        assert_eq!(m04_static::f01_static(), ());
        assert_eq!(m04_static::f02_case(), ());
    }
}
