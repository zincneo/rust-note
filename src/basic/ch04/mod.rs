fn _ch04_01_structure() {
    /// # 结构体
    /// - rust中的自定义类型
    /// - 存在以下结构体形式
    ///     1. struct DataType{} 键值对
    ///     2. struct DataType() 元组
    ///     3. struct DataType 没有属性
    fn structure() {
        #[allow(dead_code)]
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        // 手动定义每一个属性的值
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
        // 通过另外一个同类型的实例解包赋值
        // 在这里要注意属性的数据所有权问题
        let _user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };
        println!("{}", user1.email);
        fn _build_user(email: String, username: String) -> User {
            // 属性赋值缩写形式
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }
        // 类似元组的结构体
        struct Color(i32, i32, i32);
        let black = Color(0, 0, 0);
        // 访问字段的时候使用.下标的方式
        println!("{} {} {}", black.0, black.1, black.2);
        // 空结构体 后续讲到特征才有用
        struct _AlwaysEqual;
    }
    structure();
}

fn _ch04_02_using_struct() {
    /// ## 结构体的使用
    /// - 使用结构体将本该关联的数据封装起来
    /// - 特征初探->让编译器自动实现Debug特征
    fn using() {
        struct Rect(i32, i32);
        let rect1 = Rect(10, 10);
        // println宏无法直接打印结构体
        println!("{} {}", rect1.0, rect1.1);
        // Debug宏可以让println宏以调试方式打印结构体
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        // 调试方式输出占位{:?}
        // 更加可读的调试方式输出占位{:#?}
        println!("rect1 is \n{:#?}", rect1);
    }
    using();
}

fn _ch04_03_method() {
    /// ## 方法
    /// - 通过impl DataType {}来为结构体实现属于结构体的方法
    /// - 通过实例.method()进行调用
    /// - rust会调用方法的时候会自动解引用因此没有->
    /// - 实例方法的第一个参数是self
    /// - 后续章节有详细解释，这里简单了解对实例本身可以有以下获取方式
    ///     - self
    ///     - &self
    ///     - &mut self
    /// - 方法没有self参数的称为关联方法，即可以通过结构体类型名进行调用，不可以使用实例调用
    /// - 在impl块中Self表示结构体类型本身
    /// - 对于同一个结构体可以有多个impl块
    fn method() {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        #[allow(dead_code)]
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
            fn square(size: u32) -> Self {
                Self {
                    width: size,
                    height: size,
                }
            }
        }
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let area = rect1.area();
        println!("The area of the rectangle is {} square pixels.", area);
        let _rect2 = Rectangle::square(10);
    }
    method();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch04_01() {
        assert_eq!(_ch04_01_structure(), ());
    }

    #[test]
    fn ch04_02() {
        assert_eq!(_ch04_02_using_struct(), ());
    }
}
