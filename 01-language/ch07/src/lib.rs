#![allow(dead_code)]
#![allow(unused)]
/**
## 泛型参数

- 函数的泛型参数列表在函数名之后`fn function<T, U>()`
    - 泛型参数命名可以随便起，T(Type)经常作为泛型参数名
    - 调用的时候可以显示指定类型`function::<具体的类型>()`(在编译器推导不出来的时候必须写)
- 结构体的泛型参数列表在结构体名之后
    - `struct StructName<T>(T);`
    - 在创建实例的时候可以显示指定类型
    - `let variable = StructName::<i32>(0);`
- 枚举的泛型参数列表在枚举名之后
    - `enum EnumName<T> ...`
- 对于有泛型参数的结构体和枚举在实现方法时的规则
    - `impl`关键字后要提供和结构体和枚举相同的泛型参数列表
    - `impl<T> StructName<T>`
    - 方法可以又存在自身的泛型参数列表
    - 实现方法的时候可以为具体类型实现方法
        - `struct StructName<T>;`
        - 为特定类型实现方法的时候`impl`块不需要提供对于的泛型参数
        - `impl StructName::<i32>`
*/
pub fn f01_generics() {
    struct Point<T, U> {
        x: T,
        y: T,
        weight: U,
    }

    impl<T, U> Point<T, U> {
        fn new(x: T, y: T, weight: U) -> Self {
            Self { x, y, weight }
        }
    }

    impl<U> Point<i32, U> {
        fn print(&self) {
            println!("position: {}, {}", self.x, self.y);
        }
    }

    let p = Point::new(0, 0, 10usize);
    p.print();
}

/**
# const泛型
- 普通泛型针对类型，const泛型针对值
- const泛型语法`<const VAR: type>`
- 使用场景
    - 和const修饰的函数结合使用可以提高代码可读性和复用性
    - 用作安全接口，保证运行时创建的内存空间大小符合要求
    - 加密算法...
*/
pub fn f02_const_generics() {
    const fn generate_array<const N: usize>(init: usize) -> [usize; N] {
        let mut arr = [0; N];
        let mut i = 0;
        while i < N {
            arr[i] = init * (i + 1);
            i += 1;
        }
        arr
    }
    // 编译时生成固定长度，元素值指定的数组
    const ARR: [usize; 4] = generate_array(2);

    // 不使用const fn 和 const泛型
    fn generate_array_runtime(n: usize, init: usize) -> Vec<usize> {
        (1..=n).map(|i| init * i).collect()
    }
    // 运行时计算，无编译时保证，且只能得到Vec没法获得指定长度的数组
    let arr = generate_array_runtime(4, 2);

    // 安全接口
    struct Buffer<const SIZE: usize>([u8; SIZE]);

    impl<const SIZE: usize> Buffer<SIZE> {
        const fn new() -> Self {
            Buffer([0; SIZE])
        }

        // 编译时校验输入数据长度
        const fn from_array(data: [u8; SIZE]) -> Self {
            Buffer(data)
        }
    }
}

pub fn generics() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_generics() {
        assert_eq!(f01_generics(), ());
        assert_eq!(f02_const_generics(), ());
    }
}
