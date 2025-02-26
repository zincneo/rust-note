fn _ch04_01_structure() {
    /// # 结构体
    /// - rust中的自定义类型
    /// - 存在以下结构体形式
    ///     1. struct DataType{} 键值对
    ///     2. struct DataType() 元组
    ///     3. struct DataType 没有属性
    fn structure() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch04_01() {
        assert_eq!(_ch04_01_structure(), ());
    }
}
