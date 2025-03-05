use std::fmt::Display;

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

fn _ch07_02_trait() {
    /// ## 特征
    /// - rust的结构体没有继承，因此没有办法通过类似c++那样的虚函数表的方式实现多态
    ///    - 过于复杂的继承会导致分不清楚成员属性和成员方法到底是哪一个层级的属性
    ///    - 使用组合(在一个结构体内提供另外一个结构体作为成员属性)可以解决继承导致的属性和方法结构过于复杂的问题
    /// - 因此为了实现共享相同行为(多态)，rust提供了特征
    /// - 特征定义了一组可以被共享的行为，只要实现了特征，就能使用这组行为
    /// - 特征必须要遵循孤儿规则，即要为A实现特征T，则A和T至少要有一个在当前的作用域中定义
    /// - 实现特征的语法 impl trait_name for struct_name_or_enum_name {}
    /// - 特征中的函数可以只有声明也可以提供默认实现
    /// - 将特征作为函数参数 fn function_name(item: &impl trait_name)
    /// - 特征约束
    ///   - 在泛型的参数列表中使用<T: trait_name> 表示T这种类型必须实现指定的特征
    ///   - 上面的将特征作为函数参数其实就是这种写法的语法糖
    ///   - 特征约束可以是多重的，这时候可以使用+运算符表示希望被多个特征约束<T: trait_name1 + trait_name2>
    ///   - 如果约束太多导致函数签名过于复杂的时候可以使用where约束
    /// - 特征作为函数的返回值fn function_name() -> impl trait_name
    ///   - 只能返回一种具体实现的类型
    #[allow(unused)]
    fn _trait() {
        trait Summary {
            fn summarize(&self) -> String;
        }
        struct Post {
            title: String,   // 标题
            author: String,  // 作者
            content: String, // 内容
        }

        impl Summary for Post {
            fn summarize(&self) -> String {
                format!(
                    "文章{}, 作者是{}, 内容是{}",
                    self.title, self.author, self.content
                )
            }
        }

        struct Weibo {
            username: String,
            content: String,
        }

        impl Summary for Weibo {
            fn summarize(&self) -> String {
                format!("{}发表了微博{}", self.username, self.content)
            }
        }

        // Post和Weibo类型都已经实现了Summary特征，因此实例都可以使用summarize方法
        // rust通过特征来实现多态
        // 类比c++中泛型通过数组来存储父类指针，到实际使用的时候通过虚函数表来实现多态
        // rust中使用动态数组包含智能指针Box指向堆上实现了对应特征的类型来实现多态
        let mut messages = Vec::<Box<dyn Summary>>::new();
        messages.push(Box::new(Post {
            title: String::from("post"),
            author: String::from("neo"),
            content: String::from("post"),
        }));
        messages.push(Box::new(Weibo {
            username: String::from("neo"),
            content: String::from("weibo"),
        }));
        for m in messages.iter() {
            println!("{}", m.summarize());
        }

        // 特征约束
        fn _notify_0(item: &(impl Summary + Display)) {}
        // 等价写法
        fn _notify_1<T: Summary + Display>(item: &T) {}
        // 如果函数签名过长可以使用where关键字在返回值之后和函数体之前写上约束
        fn _notify_2<T>(item: &T) -> ()
        where
            T: Summary + Display,
        {
        }

        // 特征作为返回值，函数体内只有具体的一种类型
        fn returns_summarizable() -> impl Summary {
            Weibo {
                username: String::from("neo"),
                content: String::from("weibo"),
            }
        }
    }
    _trait();
}

fn _ch07_03_trait_obj() {
    /// ## 特征对象
    /// - 对于trait可以使用dyn关键字
    /// - dyn trait_name表示是堆上一个实现了该特征的对象，因此称为特征对象
    /// - dyn 只作为引用或者智能指针出现
    ///   - &dyn trait_name
    ///   - Box<dyn trait_name>
    /// - 实现方法类似c++的虚函数表，在栈上保存的数据除了指向对象的指针外还包含了指向该对象的类型对特征方法实现的指针
    /// - dyn只能作用于对象安全的特征
    ///   - 特征包含的方法返回值不能是Self
    ///   - 特征包含的方法没有任何泛型参数
    fn trait_object() {
        trait Draw {
            fn draw(&self) {
                println!("default draw");
            }
        }
        struct Button;
        struct List;
        impl Draw for Button {
            fn draw(&self) {
                println!("Button draw");
            }
        }
        impl Draw for List {
            fn draw(&self) {
                println!("List draw");
            }
        }
        impl Draw for u8 {
            fn draw(&self) {
                println!("u8 draw");
            }
        }
        fn draw_objs(objs: Vec<Box<dyn Draw>>) {
            for obj in objs.iter() {
                obj.draw();
            }
        }
        let mut objs = Vec::<Box<dyn Draw>>::new();
        objs.push(Box::new(8_u8));
        objs.push(Box::new(Button));
        objs.push(Box::new(List));
        draw_objs(objs);
    }
    trait_object();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch07_01() {
        assert_eq!(_ch07_01_generics(), ());
    }

    #[test]
    fn ch07_02() {
        assert_eq!(_ch07_02_trait(), ());
    }

    #[test]
    fn ch07_03() {
        assert_eq!(_ch07_03_trait_obj(), ());
    }
}
