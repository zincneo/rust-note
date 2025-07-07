#![allow(dead_code)]
/**
# 引用
- Rust中通过借用来避免所有权转移导致代码复杂度上升
- 获取变量的引用，称之为借用(borrowing)
- 引用(reference)和借用(borrowing)就是一回事，Rust又拿了个新词表述罢了
- 在变量名前面加上&即可获得该变量的借用
- 常规引用是一个指针类型，指向了对象存储的内存地址
- 使用`let ref_variable = &value`，可以获取变量的引用，*borrowing_name可以解除引用获取值
- 使用`let ref variable = value`, 可以获得变量的引用
- 另外一种获取引用的方式是定义变量的时候使用ref关键字，这时候不能加上&运算符
- &variable_name称为不可变的引用，无法通过引用修改变量的值
- 不可变引用可以同时存在多个
```
*/
pub fn f01_ref() {
    let num = 30;
    let r_num = &num;
    let ref _r_num = num; // 可以同时存在多个不可变引用
    assert_eq!(num, *r_num);
}

/**
# 可变引用
- `let ref_variable = &mut variable_name`称为获取一个变量的可变引用
- 另外一种形式为`let ref mut ref_variable = variable_name;`
- 可变引用同一时刻只能存在一个，且不可以和不可变引用同时存在
- 注意当前编译器版本下，引用作用域的结束位置从花括号变成最后一次使用的位置
*/
pub fn f02_mut_ref() {
    let mut num = 100;
    let ref _r_num = 100; // 不会导致同时存在不可变引用和可变引用的错误，因为编译器会检查最后一次使用的位置
    let ref mut r_num = num;
    *r_num += 10;
    println!("{r_num}");
}
