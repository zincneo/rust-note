#![allow(dead_code)]
#![allow(unused)]

/**
# 关联类型
- 关联类型是一种通过 trait 定义的高级特性
- 允许在特征中定义与实现类型相关的类型别名，从而减少泛型参数的数量
- 在特征代码块中使用`type TypeName`关键字定义关联类型
- 在特征内使用关联类型必须使用`Self::TypeName`来消除歧义
- 简化代码的复杂性，并提高可读性和灵活性
- impl特征的时候指定对应的类型
- 相比于泛型参数来说使用关联类型只能有固定的类型指定，不想特征泛型参数可以有多个实现
*/
pub fn f01_associate_type() {
    // 设计这样的需求
    // 要表示不同种类的动物，每种动物都具有吃东西的能力，但是不同种类的动物需要的食物不同
    // 1. 不使用关联类型
    {
        struct Cow;
        struct Tiger;
        // 在特征上使用泛型参数
        trait Animal<Food> {
            fn eat(&self, food: Food);
            fn preferred_food() -> Food;
        }
        struct Hay;
        struct Meat;
        impl Animal<Hay> for Cow {
            fn eat(&self, food: Hay) {
                println!("Cow eats hay");
            }
            fn preferred_food() -> Hay {
                Hay
            }
        }

        impl Animal<Meat> for Tiger {
            fn eat(&self, food: Meat) {
                println!("Tiger eats meat");
            }
            fn preferred_food() -> Meat {
                Meat
            }
        }
        // 缺点
        // 1. 类型不一致，如Cow可以同时实现Animal<Hay>和Animal<Meat>
        impl Animal<Meat> for Cow {
            fn eat(&self, food: Meat) {
                println!("Cow eats meat");
            }
            fn preferred_food() -> Meat {
                Meat
            }
        }
        // 2. 函数签名复杂，使用特征的时候还要携带泛型参数
        fn feed<T, F>(animal: T)
        where
            T: Animal<F>,
        {
            animal.eat(T::preferred_food());
        }
    }
    // 2. 使用关联类型
    {
        trait Animal {
            // 在特征定义中使用type关键字来表示关联类型
            type Food;

            fn eat(&self, food: Self::Food);
            fn preferred_food() -> Self::Food;
        }
        struct Cow;
        struct Tiger;
        struct Hay;
        struct Meat;
        impl Animal for Cow {
            // 实现特征的时候指定具体的关联类型是什么
            type Food = Hay;
            fn eat(&self, food: Self::Food) {
                println!("Cow eats Hay");
            }
            fn preferred_food() -> Self::Food {
                Hay
            }
        }
        impl Animal for Tiger {
            type Food = Meat;
            fn eat(&self, food: Self::Food) {
                println!("Tiger eats Meat");
            }
            fn preferred_food() -> Self::Food {
                Meat
            }
        }
        // 优点，同样功能的函数可以省略泛型参数
        fn feed<T>(animal: T)
        where
            T: Animal,
        {
            animal.eat(T::preferred_food());
        }
    }
}

/**
# 模拟迭代器的例子
*/
pub fn f02_case() {
    // 1. 模拟类似迭代器的特征
    trait CutomIter {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    // 2. 元素类型为泛型的结构体
    struct Custom<T: Copy + std::ops::Add + std::ops::AddAssign>(T, usize);
    // 3. 实现特征
    impl<T> CutomIter for Custom<T>
    where
        T: Copy + std::ops::Add + std::ops::AddAssign,
    {
        // 关联类型绑定到结构体的泛型参数上
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            match self.1 {
                0..10 => {
                    let res = Some(self.0);
                    self.0 += self.0;
                    self.1 += 1;
                    res
                }
                _ => None,
            }
        }
    }
    let mut custom = Custom(10, 0);
    while let Some(n) = custom.next() {
        println!("{n}");
    }
}
