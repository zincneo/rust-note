fn _ch07_01_generics() {
    /// ## 泛型
    /// - 泛型就是一种多态
    /// - rust使用<>来作为泛型参数列表
    /// - 泛型的常见使用形式
    ///   - 结构体或者枚举的类型名后使用泛型参数列表
    ///   - 函数名后使用泛型参数列表
    ///   - impl和类型名后都要加上泛型参数列表
    ///   - 为具体的某一个类型实现特定方法的时候impl关键字后不需要泛型参数列表，类名后面指明具体类型
    #[allow(dead_code)]
    fn generics() {
        // 枚举和结构体使用泛型参数列表
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }
        let integer = Point { x: 1, y: 2 };
        let float = Point { x: 1.0, y: 2.0 };
        println!("{:?}", integer);
        println!("{:?}", float);
        // 函数使用泛型参数列表
        fn get_num<T>(i: T) -> T {
            i
        }
        println!("{}", get_num(10));
        // 方法使用泛型参数列表
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        println!("{}", integer.x());
        impl Point<f64> {
            fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        println!("{}", float.distance_from_origin());
    }
    generics();

    /// ## const泛型
    /// - 前面的泛型是针对类型的泛型
    /// - const泛型则是针对值的泛型
    /// - 泛型参数列表中的写法<..., const value_name: type>
    fn const_generics() {
        // rust中数组为固定长度，通过const泛型来兼容元素类型为i32，长度任意的数组
        fn display_array<const N: usize>(arr: [i32; N]) {
            println!("{:#?}", arr);
        }
        let arr: [i32; 3] = [1, 2, 3];
        display_array(arr);
    }
    const_generics();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch07_01() {
        assert_eq!(_ch07_01_generics(), ());
    }
}
