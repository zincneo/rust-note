/**
# 模式使用场景

1. 匹配字面值

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

2. 匹配命名变量

```rust
let x = Some(5);
let y = 10;
match x {
    Some(50) => println!("Got 50"),
    // y产生了变量遮蔽，这里不绑定到外部的变量y的值而是绑定到还未列出的任意i32值
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}
println!("at the end: x = {:?}, y = {:?}", x, y);
```

3. 匹配Range语法生成的序列
    - `start..end` `[start, end)`
    - `start..=end` `[start, end]`

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

4. 匹配单分支多模式
    - `|`用来在模式中表达或，在一个分支中匹配多个pattern

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

5. 匹配守卫提供额外的匹配条件
    - 模式之后的一个if条件，用一个返回布尔值的表达式来设置更多条件
    - 匹配守卫在单分支多模式的情况下会作用于所有模式

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}

let x = 4;
let y = false;

match x {
    // if y 会作用于 4, 5, 6三个模式 (4 | 5 | 6) if y
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

6. @绑定
    - 将一个字段@绑定到另外一个变量(多模式)
    - 绑定的同时解构

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 绑定新变量 `p`，同时对 `Point` 进行解构
let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
println!("x: {}, y: {}", px, py);
println!("{:?}", p);

let point = Point {x: 10, y: 5};
if let p @ Point {x: 10, y} = point {
    println!("x is 10 and y is {} in {:?}", y, p);
} else {
    println!("x was not 10 :(");
}

// 通过@绑定到单分支多模式，需要使用()保证优先级
match 1 {
    num @ (1 | 2) => {
        println!("{}", num);
    }
    _ => {}
}
```

7. 解构和忽略(见下一节)
*/
pub fn scence() {
    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
