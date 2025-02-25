fn _ch02_01_variable_mutability() {
    /// ## rust变量
    /// 1. 关键字let
    /// 2. rust变量默认不可以在定义后改变值
    /// 3. 编译器会警告未使用的变量
    /// 4. 变量名使用_开头可以关闭对该变量未使用的警告
    /// 5. 如果希望变量可变需要加上mut关键字
    fn variabel_mutability() {
        let _v = 10;
        let mut _v = 10;
        _v += 2;
    }
    variabel_mutability();

    /// ## 变量遮蔽
    /// 1. rust支持同名变量在同一作用域重复定义
    /// 2. 后定义的会遮蔽前面定义的
    /// 3. 对于后定义但变量生命周期先结束的，结束后可以再使用之前定义的同名变量
    fn shadowing() {
        let _v = 10;
        println!("{_v}");
        let _v = 20;
        println!("{_v}");
        {
            let _v = 30;
            println!("{_v}");
        }
        println!("{_v}");
    }
    shadowing();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch02_01() {
        assert_eq!(_ch02_01_variable_mutability(), ());
    }
}
