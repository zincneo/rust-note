#![allow(dead_code)]
/**
# for循环
- Rust提供了for关键字用来实现循环
- Rust中的for循环本质只是一个迭代器的语法糖，具体看迭代器章节
- `for i in start..end`
    - Rust提供`start..end`语法来生成一个序列
    - `..`返回的是一个[)序列，`..=`返回的是一个[]序列
- `for (index, val) in array.iter().enumerate()`
    - 设计迭代器和特征详细见后续的章节，总之enumerate会返回`(索引,值)`的迭代器
*/
pub fn f01_for() {
    let range = 1..10;
    for i in range {
        println!("{i}");
    }
    let range = 1..=10;
    for i in range {
        println!("{i}");
    }
    let arr = [1, 2, 3];
    for (index, val) in arr.iter().enumerate() {
        println!("{index}: {val}");
    }
}

/**
# loop和while循环
- Rust提供了loop关键字实现循环，循环本身不提供终止条件，何时终止取决于内部是否有会执行break语句
    - loop也是一个表达式，是有返回值的，该返回值由循环体的break进行返回，或者无法结束的发散类型`!`也可以看做一种类型
- Rust提供了while关键字实现循环，`while expression`，每次执行循环体之前都会执行一次表达式，当返回值为`false`时终止循环
    - while作为表达式则固定返回空元组
*/
pub fn f02_loop_while() {
    let mut n = 0;
    while n < 3 {
        n += 1;
        println!("{}", n);
    }
    fn fn_loop() {
        let _forever = loop {
            println!("forever");
        };
    }
}

/**
# break和continue
- 和其他语言的设计相同，break用来跳出当前循环，continue用来跳过本次循环
- 这两个关键字必须作用于for, loop, while的作用域内
*/
pub fn f03_break_continue() {
    for i in 0..100 {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}");
    }

    for i in 0..1000_0000_000_usize {
        if i % 33 == 1 && i % 7 == 1 {
            break;
        }
    }

    // 跳过break返回值
    let _n = loop {
        break 3;
    };
}

/**
# label语法
- Rust提供`'label_name: for/while/loop`语法来给循环体打上标记
- 作用是配合break语句精确指定要跳出那一层循环
*/
pub fn f04_label() {
    let mut num = 5;
    let _n = 'out: loop {
        'inner: loop {
            if num % 2 == 0 {
                num += 1;
                break 'inner;
            }
            break 'out 10;
        }
    };

    'out: for i in 0..10 {
        let mut t = i;
        'inner: while t % 2 == 0 {
            t = t * 3 - 1;
            if t % 5 == 2 {
                break 'out;
            } else {
                break 'inner;
            }
        }
    }
}
