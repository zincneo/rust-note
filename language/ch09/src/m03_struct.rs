#![allow(dead_code)]
#![allow(unused)]

/**
# 结构体中的生命周期

- 结构体字段的类型是引用类型的时候也需要生命周期标注
- 同样需要在结构体的泛型参数列表中需要先声明生命周期标识符
- 比较复杂的是自引用类型，见后续章节
- 与泛型参数相同，结构体的生命周期标识符在impl实现方法的时候也需要在impl块之后的泛型参数列表中先声明
*/
pub fn f01_struct() {
    // 1. 结构体包含引用类型字段
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    // 2. 应用生命周期消除规则
    impl<'a> ImportantExcerpt<'a> {
        // 在没有标明参数列表生命周期标识符的情况下，自动跟随`&self`
        fn announce_and_return_part_1(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // 3. 手动标明生命周期标识符
    impl<'a> ImportantExcerpt<'a> {
        // 手动指定返回值的生命周期标识符'b
        fn announce_and_return_part_2<'b>(&'a self, announcement: &'b str) -> &'b str {
            println!("Attention please: {}", announcement);
            announcement
        }
    }

    let message = "test";

    let i = ImportantExcerpt { part: message };

    println!("{}", i.part);
}
