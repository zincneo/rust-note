#![allow(dead_code)]

/**
# 变量
- Rust中使用let关键字定义变量
- Rust变量命名规则
    - 字母、数字和下划线组合
    - 不允许以数字作为第一个字符
    - 编译器会警告没有使用的变量
    - 使用_开头的变量名编译器不警告该变量没有使用
    - 变量名遵循小写蛇形规则(snake_case)
- Rust是强类型语言，变量有唯一固定的类型，编译器可以依据字面值推导最可能的类型
- 变量手动指定类型的语法`let variable_name : type`
- 变量的声明和值绑定可以分离
*/
pub fn f01_variable() {
    let _num = 100;
    let _num;
    _num = 0xff_usize;
}

/**
# 静态变量
- Rust中使用const关键字来定义静态变量
- 静态变量必须手动指定类型且声明和值绑定不可用分离
- 静态变量名遵循大写蛇形规则(SNAKE_CASE)
*/
pub fn f02_const_variable() {
    const I32_VAR: i32 = 20;
}
