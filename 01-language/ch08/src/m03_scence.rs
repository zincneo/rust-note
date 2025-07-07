#![allow(dead_code)]
#![allow(unused)]
/*!
# 所有的模式的使用场景
- 除了上节中的模式，以下模式通常只在match/if let/while let中比较常用，因为他们大部分是可驳的
    1. 匹配range生成的序列
    2. 使用|运算符将多个模式变成一个模式
    3. 使用if关键字来为模式提供限定条件
    4. 使用@运算符将一个变量模式或者解构模式的组合绑定到一个模式
*/

/**
# range模式
- 序列语法作为表达式返回其实是实现了迭代器相关特征的类型的值
- 序列语法作为模式则在匹配的时候用于表示该序列中可能的任意一个值
*/
pub fn f01_range_pattern() {
    // range语法做表达式返回的具体类型Range<T>的值
    let tmp = 0..10;
    match 3 {
        // 0..10序列在这里表示[0,10]中任意一个值
        0..=10 => (),
        _ => (),
    }
    // 在let else中可以使用
    let 0..=10 = 5 else {
        return;
    };
}

/**
# | 合并模式
- 用来将多个模式组合，匹配中其中一个就算匹配，算是模式匹配中的||运算符
- 只能在match/if let/while let中使用，let else也不支持
*/
pub fn f02_composition_pattern() {
    let num = 10;
    match num {
        1 | 2 | 5 => (),
        10..=20 | 100 => (),
        _ => (),
    }
    // 在let else中也无法使用，编译器会报错
    /*
    let 2 | 3 | 4 = 5 else {
        return;
    };
    */
}

/**
# if 匹配守卫
- 用来额外提供条件，if后只支持返回布尔值的表达式
- 只能在match中使用，if let和while let都不支持
*/
pub fn f03_if_guard_pattern() {
    let num = 10;
    match num {
        // 利用匹配守卫匹配偶数，结合了变量模式使用
        x if x % 2 == 0 => {}
        _ => (),
    }
}

/**
# @ 绑定
- 将一个模式绑定到另外一个模式，通常和解构模式和字面值模式配合使用
- if let/ while let 支持该语法
*/
pub fn f04_at_pattern() {
    struct Custom {
        arr: [i32; 5],
        desc: String,
        message: (i32, u64, f32),
    }

    let custom = Custom {
        arr: [2, 3, 4, 5, 0],
        desc: "custom".to_string(),
        message: (0, 1, 0.1),
    };

    match custom {
        Custom {
            // 将nums在解构的时候绑定到另外一个模式，该模式中产生的变量模式同样可以使用
            arr: nums @ [2, x, ..],
            desc,
            message: message @ (0, y, _),
        } => {
            println!("{:?}, {}, {}", nums, x, y);
        }
        _ => (),
    }

    // if let 支持该语法
    if let x @ 3 = 3 {};
}
