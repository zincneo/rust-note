#![allow(dead_code)]
#![allow(unused)]
/**
# 创建和使用Vec

- 创建动态数组
    1. 关联方法`Vec::new`
    2. 宏`vec![]`
    3. 当元素类型是特征对象的情况下必须手动注明变量类型
- 当数组变量丢弃的时候元素也被丢弃
- 访问数组
    1. 使用`[]`访问，直接访问元素，越界会导致不可恢复的崩溃
    2. 使用`get`方法访问，返回一个Option枚举
- 元素同样遵顼所有权原则
    1. 使用`get`方法访问，对于无法发生Copy的元素类型会获取该包裹元素不可变引用的Option
    2. 使用`[]`访问，对于无法发生Copy的元素类型会导致无法通过编译
- 常用方法
    - push 在末尾追加元素
    - insert
    - reserve 调整容量
    - shrink_to_fit 释放剩余的容量，一般情况下，不会主动去释放容量
    - is_empty
    - remove
    - pop 删除最后一个元素并作为值返回(返回的是Option包裹的值)
    - 数组的to_vec方法 将数组转换为vec
    - append
    - truncate 截断到指定长度，多余的元素被删除
    - retain 传入一个返回值为布尔值的闭包，保留满足条件的元素，即删除不满足条件的元素
    - drain 传入一个Range删除指定范围的元素，同时获取被删除元素的迭代器
    - split_off 指定索引处切分成两个，返回的是后半段
*/
pub fn f01_vec() {
    let mut v = Vec::new(); // 编译器依据下一行推出来类型为Vec<i32>
    v.push(1);
    // 2. vec!宏
    let mut v = vec![1, 2, 3];
    // 3. 更新值push_back
    v.push(1);
    // 4. 当Vec类型的值离开作用域的时候内部的元素一同被drop
    {
        let v = vec![1, 2, 3];
    } // drop v和内部元素

    // 5. 读取v中的元素
    // 使用[]运算符访问，该方式越界访问会导致panic
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("third is {}", third);
    // 使用get方法访问，该方法返回一个Option，越界返回None
    match v.get(2) {
        Some(third) => println!("third is {third}"),
        None => println!("third is None"),
    }

    // 6. 注意所有权问题
    // 借用数组中的某一个元素的时候要小心方法的调用(方法本身也要获取数组的引用)
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 创建不可变借用(借用部分也视同对整个动态数组产生了借用)
    v.push(6); // 创建可变借用
    // println!("The first element is: {first}"); 报错，因为同时使用了不可变借用和可变借用
    let mut v = vec!["test".to_string()];
    let a = v.get(0);

    // 7. 迭代所有元素
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10
    }
}

/**
# 排序

1. sort,sort_unstable
    - 这两个方法依赖数组的元素实现`Ord`、`PartialOrd`、`Eq`、`PartialEq`特征
2. sort_by,sort_unstale_by
    - 这两个方法需要传入一个返回布尔值的闭包来决定如何排序
3. unstable和非unstable的差异在于对相等元素的处理，非unstable的sort和sort_by在排序过程中不会导致相等元素重排
*/
pub fn f02_sort() {
    // 1. 整数排序
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    // 2. 浮点数排序
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // vec.sort_unstable(); 报错，因为浮点数没有实现Ord特征，存在一个不能比较的值NAN
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
