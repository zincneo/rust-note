#![allow(dead_code)]
#![allow(unused)]
/*!
# 解构语法
- 解构语法本身就是模式
- 常见的解构
    1. 数组/切片
    2. 元组
    3. 结构体
    4. 元组
- 归属可驳还是不可驳要看具体的使用方式
*/

/**
# 元组解构
- 通常和变量模式，忽略模式，剩余模式嵌套使用
*/
pub fn f01_tuple() {
    let (a, b, .., _) = (1, 2, 'c', 1.1);
}

/**
# 结构体解构
- 结构体解构的时候默认使用字段名作为变量模式的名称
- 如果要指定字段到自定义的变量名需要使用`prop_name: variable_name
*/
pub fn f02_struct() {
    struct Custom {
        x: i32,
        y: i32,
    }
    let Custom { x, y } = Custom { x: 10, y: 20 };
    let Custom { x: new_x, y: new_y } = Custom { x: 20, y: 10 };
}

/**
# 解构数组
- 解构数组的时候要注意不定长的切片很经常使用，语法和数组又相同，但是切片只能适用用可驳的匹配
*/
pub fn f03_array() {
    let num = [1, 2, 3];
    let [a, b, c] = num;
    // 使用切片必须用let else
    let [a, b, c] = num[..] else {
        return;
    };
}

/**
# 解构枚举
- 解构枚举同样枚举值为可驳，因此只能使用与可驳的匹配，let和函数参数是不可以使用的
*/
pub fn f04_enum() {
    enum Message {
        Color(i32, i32, i32),
        Point(i32, i32),
    }

    let Message::Point(x, y) = Message::Point(10, 10) else {
        return;
    };
}
