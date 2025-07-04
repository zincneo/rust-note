#![allow(unused)]
#![allow(dead_code)]

/**
# 特征定义和实现

- Rust提供trait关键字用以实现特征
    - 特征的作用是提供多态功能
    - 实现了相同特征的结构体和对象可以调用同名的方法
- 定义特征使用`trait TraitName {}`
- 实现特征使用`impl TraitName for StructOrEnumName {}`
- 在定义特征时可以提供方法的默认实现
    - 在实现特征的时候可以使用默认的也可以选择重载特征中的方法
- 为类型A实现特征T，那么A和T至少有一个在当前作用域中定义
    - 该规则称为**孤儿规则**，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码
*/
pub fn f01_trait() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub struct Post {
        pub title: String,   // 标题
        pub author: String,  // 作者
        pub content: String, // 内容
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }
}

/**
# 特征对象
- 想要使用一个具有某种特征但不明确具体类型的值时有两种方法
    1. 特征约束`impl TraitName`后续章节有更多细节解释
    2. 特征对象`Box<dyn TraitName>`
- 实现某一类特征的对象称为该类特征的特征对象
- 作用是当编码需求不固定的类型只要求他们又共同的行为(多态)时使用
    - 例如不关心具体的类型，函数只需要传入的参数可以调用某个名称的方法时使用
- 特征对象分配在堆上`Box<dyn TraitName>`，这里用到了智能指针Box，指向堆上一个实现了指定特征的对象，这种方式指向的实际对象类型可以不同
*/
pub fn f02_trait_object() {
    trait A {
        fn echo(&self) {
            println!("message");
        }
    }
    struct Triangle;
    struct Rectangle;
    impl A for Triangle {}
    impl A for Rectangle {}

    // 特征对象用于函数的返回值，做参数也是同理
    {
        fn func_impl() -> impl A {
            Triangle // 不允许通过流程控制返回不同类型
        }

        fn func_dyn(status: bool) -> Box<dyn A> {
            if status {
                Box::new(Triangle)
            } else {
                Box::new(Rectangle)
            }
        }
    }

    // 通过数组保存一组特征对象，体现多态的用处
    // 这里编译器无法推导，需要手动标注类型
    let shapes: [Box<dyn A>; 2] = [Box::new(Triangle), Box::new(Rectangle)];
    for shape in shapes {
        shape.echo();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_trait() {
        assert_eq!(f01_trait(), ());
        assert_eq!(f02_trait_object(), ());
    }
}
