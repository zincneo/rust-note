/**
# <span style="color: #a6da95;">解构和忽略</span>

## 解构语法

可以使用模式来解构结构体、枚举、元组、数组和引用

1. 结构体解构

```rust
// 1. let绑定的时候使用pattern解构结构体
struct Point {
    x: i32,
    y: i32,
}
let p = Point { x: 0, y: 7 };
let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(7, b);
// 如果变量名和字段名相同可以简写为
let Point { x, y } = p;
assert_eq!(0, x);
assert_eq!(7, y);

// 2. match在模式中解构结构体
let p = Point { x: 0, y: 7 };
match p {
    // 解构的同时结合绑定到字面值使用
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

2. 解构枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

// 1. match在模式中解构枚举
match msg {
    Message::Quit => {
        println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
        println!(
            "Move in the x direction {} and in the y direction {}",
            x,
            y
        );
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
        println!(
            "Change the color to red {}, green {}, and blue {}",
            r,
            g,
            b
        )
    }
}

// 2. let为不可驳，因此要使用let else来解构枚举
let message = Message::ChangeColor(255, 0, 0);
let Message::ChangeColor(_a, _b, _c) = message else {
    return;
};
```

3. 解构数组

```rust
// 1. let绑定的时候使用解构数组
let arr: [u16; 2] = [114, 514];
let [x, y] = arr;

assert_eq!(x, 114);
assert_eq!(y, 514);

// 不定长的数组(切片)结合..来解构
// 注意要使用可驳的if let或者let else
let arr: &[u16] = &[114, 514];
if let [x, ..] = arr {
    assert_eq!(x, &114);
}
if let &[.., y] = arr {
    assert_eq!(y, 514);
}

```

4. 解构元组

```rust
// 1. let绑定中使用元组解构
// 和其他解构可以嵌套使用
struct Point {
     x: i32,
     y: i32,
 }
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

## 忽略语法

1. _用来忽略整个值

```rust
// 作为match的最后一个分支
match 1 {
    1 => (),
    _ => ()
}
```

2. 嵌套地使用_忽略部分值

```rust
// 通常是在解构的时候用来忽略某个位置的值
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);

let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

3. 使用_在let绑定的时候让编译器不要警告没有使用的变量

```rust
let _x = 5;
```

4. 使用..忽略其他部分的值，语义必须明确

```rust
// 1. 结构体中忽略其他值
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}

// 2. 在元组中间使用
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
    },
}
```

*/
#[allow(dead_code)]
pub fn f02_03_deconstruction() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let message = Message::ChangeColor(255, 0, 0);
    let Message::ChangeColor(_a, _b, _c) = message else {
        return;
    };
}
