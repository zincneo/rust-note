#![allow(dead_code)]
#![allow(unused)]
/*!
# 模式
- Rust中模式用来一个和表达式的返回值或者类型进行匹配
    - `let x = 10;` 这个语句rust认为x是一个模式，而10是一个表达式的返回值
    - `fn func(x: i32);` 这个函数声明认为x是一个模式，而i32是一个类型
- 模式可驳
    - 模式分为可驳(refutable)和不可驳(irrefutable)
    - 不可驳模式
        - 总是能够成功匹配的模式，即不会失败
        - 适用于那些必须匹配成功的地方
        - 例如用变量匹配一个表达式的返回值，这总是能够成功的`let x = 5`,而字面值`3`匹配表达式就不一定能成功`let 3 = 5`
    - 可驳模式
        - 可能匹配失败的模式。当值可能不符合模式时，匹配就会失败
        - 适用于那些需要处理不同可能性的地方
        - 字面值或者字面值与其他任意模式的组合
- Rust模式(pattern)通常由以下内容单独构成或者组合构成
    1. 字面值(可驳)
    2. 解构的数组、枚举、结构体或者元组(本身是组合，驳回性要看组合内容而定)
    3. 变量(不可驳)
    4. 通配符`_`(不可驳)
    5. 占位符`..`(不可驳)
- 使用模式的场景
    1. match `match expression {pattern => ...}`
    2. if let `if let pattern = expression {}`
    3. while let `while let pattern = expression {}`
    4. for `for pattern in impl Iterator {}`
    5. let语句 `let pattern = expression`
    6. 函数参数 `fn function(pattern: type)`
*/

/**
# match中使用模式
- if let和while let用法基本上完全一致
*/
pub fn f01_match_pattern() {
    // 1. 字面值
    // 4. 通配符
    match 5 {
        0 => (),
        _ => (),
    }

    // 2. 解构数组
    // 5. 占位符
    let arr = [2, 3, 5];
    match arr {
        [2, 3, x] => (),
        [2, ..] => (),
        _ => (),
    }

    // 3. 变量
    let x = 10;
    match 10 {
        x => println!("{x}"),
        _ => (),
    }
}

/**
# for中使用模式
- `for pattern in ITERATOR {}`
- pattern用来匹配Iterator特征的Item
*/
pub fn f02_for_pattern() {
    let v = vec![('a', 10), ('b', 20), ('c', 30)];
    let iter = v.iter().enumerate();

    for (index, (c, _)) in iter {
        println!("{index}: {c}");
    }
}

/**
# let变量绑定时使用模式
- let绑定必须使用不可驳的模式
- `let patten = expression;`
*/
pub fn f03_let_pattern() {
    // 1. 字面值 - 不允许，let绑定不能使用可驳模式
    // let 5 = 5; 编译器报错
    // 2. 解构的数组、枚举、结构体或者元组
    let (a, b, c) = ('a', 5, 1.1);
    // 3. 变量
    let n = 5;
    // 4. 通配符`_`
    let (_, _, a) = (1, 2, 3);
    let _ = 1;
    // 5. 占位符`..`
    let [.., c] = [1, 2, 3, 4, 5];
}

/**
# 函数参数中使用模式
- 函数中使用的模式必须是不可驳
*/
pub fn f04_argument_pattern() {
    // 1. 字面值 - 不可以使用，因为是可驳的
    // 2. 解构的数组、枚举、结构体或者元组，组合之后必须是不可驳的
    // 下面例子的模式组合了元组解构和变量(a, b, c)其对于类型(i32, i32, T)一定能匹配成功，因此是不可驳的模式
    fn f01<T>((a, b, c): (i32, i32, T)) {}
    // 3. 变量
    // 4. 通配符`_`
    // 5. 占位符`..`
    fn f02([a, .., b, _]: [i32; 10]) {}
}

/**
# let else
- 一般的let绑定只支持不可驳的模式
- `let pattern = expression else {};`
- else块用来处理不匹配的情况，终止当前代码流，要么return结束当前的函数，要么panic使得程序崩溃否则无法通过编译器
*/
pub fn f05_let_else_pattern() {
    // 1. 字面值
    let 5 = 5 else {
        return;
    };
    // 2. 解构的数组、枚举、结构体或者元组
    // 这个例子本身是变量+解构结构体，但是结构体在包裹的时候已经是部分的字面值，因此模式是可驳的
    let Some(x) = Some(5) else {
        return;
    };
    // 3/4/5本身是不可驳的，因此一定不会进入else块，除非和1/2组合
}
