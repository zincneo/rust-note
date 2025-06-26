#![allow(dead_code)]
#![allow(unused)]

/**
# 方法中的生命周期
- 生命周期标识符的消除三原则
    1. 每一个引用类型的参数都会获得一个独立的生命周期
    2. 当只有一个引用类型的参数的时候，该参数的生命周期自动赋给返回值
    3. 如果方法的第一个参数是`&self`或者`&mut self`，则默认将其生命周期赋给返回值
*/
pub fn f01_method() {
    struct Custom {
        name: String,
        age: u32,
    }
    impl Custom {
        // 1. 默认会将&self的生命周期赋给返回值
        fn name(&self) -> &String {
            &self.name
        }

        // 2. 手动指定返回值的生命周期
        fn echo<'a>(&self, hello: &'a mut String) -> &'a mut String {
            hello.push(' ');
            hello.push_str(self.name.as_str());
            hello
        }

        // 3. 关联函数规则和正常的函数相同
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
    }
}
