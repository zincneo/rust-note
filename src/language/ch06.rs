/*!
# <span style="color: #ed8796;">ch06 Rust模式匹配</span>

## 01 match和if let

1. [match](./fn.f01_01_match.html)
2. [if let](./fn.f01_02_if_let.html)
3. [matches!](./fn.f01_03_matches.html)
4. [解构Option枚举](./fn.f01_04_option.html)

## 02 模式

1. [pattern](./fn.f02_01_pattern.html)
2. [模式解构](./fn.f02_02_pattern_deconstruction.html)
3. [模式使用场景](./fn.f02_03_scene.html)
*/

use core::panic;

/**
# <span style="color: #a6da95;">match</span>
- Rust提供match关键字用来将一个值和一系列值进行匹配
```
// 通用的形式
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```
- match本身是一个表达式，每一个模式对应的代码块最后一个语句就是其返回值
- match宏必须是穷尽匹配的，也就是要匹配该类型的所有值
- `_`作为模式的时候是通配符，表示剩下所有的没有列出的可能性
- 以下看几个匹配的例子
```rust
    // 匹配枚举值
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);
```
```rust
    // 匹配整数值
    let num = 10;
    match num {
        1 => {
            println!("1");
        }
        _ => (),
    }
```
- 在匹配枚举包裹值的情况下，可以通过模式取出包裹的值
    - 枚举值定义`EnumValue(type1, type2, type3)`则对应的模式为`EnumValue(name1, name2, name3)`
```
    enum Message {
        Quit,
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let message = Message::ChangeColor(255, 0, 122);
    match message {
        Message::Quit => (),
        Message::Write(s) => {
            println!("{s}");
        }
        Message::ChangeColor(_r, _g, _b) => (),
    }
```
*/
#[allow(dead_code)]
pub fn f01_01_match() {
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);
    let num = 10;
    match num {
        1 => {
            println!("1");
        }
        _ => (),
    }
    enum Message {
        Quit,
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let message = Message::ChangeColor(255, 0, 122);
    match message {
        Message::Quit => (),
        Message::Write(s) => {
            println!("{s}");
        }
        Message::ChangeColor(_r, _g, _b) => (),
    }
}

/**
# <span style="color: #a6da95;">if let</span>
- if let用于只想匹配一种模式的情况
- `if let 模式 = 值 {}`
- 很适合适用于不能使用`==`运算符判等的类型替代if
- 支持else块
```rust
    let num = Some(5);
    if let Some(3) = num {
        println!("3");
    } else {
        println!("other number");
    }
```
*/
pub fn f01_02_if_let() {
    let num = Some(5);
    if let Some(3) = num {
        println!("3");
    } else {
        println!("other number");
    }
}

/**
# <span style="color: #a6da95;">matches宏</span>
- 用来将一个表达式和一个模式进行匹配
- 匹配结果返回true和false
- 同样很适合那些没有实现`==`类型使用
- 下面的例子使用了迭代器和闭包，作用是根据filter传入的闭包来筛选想要的值，这个闭包需要返回一个布尔值
```rust
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let _v: Vec<MyEnum> = v
        .into_iter()
        .filter(|ele| matches!(ele, MyEnum::Foo))
        .collect();
```
*/
pub fn f01_03_matches() {
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let _v: Vec<MyEnum> = v
        .into_iter()
        .filter(|ele| matches!(ele, MyEnum::Foo))
        .collect();
}

/**
# <span style="color: #a6da95;">解构Option枚举</span>
- 由于Option过于常用，因此Rust在prelude中进行了导出，因此可以直接使用Some和None，不需要使用Option::Some,Option::None
- Rust通过模式匹配的方式来解决其他语言中null异常的问题
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 不存在值的情况就不会触发处理逻辑
        Some(i) => Some(i + 1), // 能够匹配到实际的对象再调用方法
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
*/
pub fn f01_04_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,           // 不存在值的情况就不会触发处理逻辑
            Some(i) => Some(i + 1), // 能够匹配到实际的对象再调用方法
        }
    }
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

/**
# <span style="color: #a6da95;">模式</span>

## <span style="color: #a6da95;">字面值</span>

- 模式可以是字面值

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

## <span style="color: #a6da95;">命名变量</span>

- 模式可以用变量占位，匹配任意值

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    // 块中产生变量遮蔽，使用命名变量y来匹配Some()剩余的所有可能性，在该模式的代码块中可以使用变量y
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}
```

## <span style="color: #a6da95;">序列语法生成的范围</span>

- `start..=end` 语法允许匹配一个闭区间序列内的值，`start..end` 语法允许匹配一个左闭右开区间序列内的值

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    6..10 => println!("six through nine"),
    _ => println!("something else"),
}
```

## <span style="color: #a6da95;">忽略模式中的某一个值</span>

- _忽略模式不只可以用在match中，其他场景如解构赋值，函数参数等都可以使用

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
let mut setting_value = Some(5);
let new_setting_value = Some(10);
```

- 并且可以嵌套地忽略部分

```rust
match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}
let numbers = (2, 4, 8, 16, 32);
// 忽略元组中指定位置的值
match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

- ..在语义明确的情况下可以忽略元组和结构体中的值

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}

let numbers = (2, 4, 8, 16, 32);

// 使用..忽略元组中间的值
match numbers {
    (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
    },
}
```

## <span style="color: #a6da95;">匹配守卫</span>

- 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件

```rust
let num = Some(4);
match num => {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

## <span style="color: #a6da95;">@绑定</span>

- @运算符允许为一个字段绑定另外一个变量
- 使用 @ 还可以在绑定新变量的同时，对目标进行解构，见模式解构

```rust
let num = Some(3);
match num {
    Some(x @ 3..=7) => {
        println!("3..=7: {x}");
    }
    _ => (),
}

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
```

## <span style="color: #a6da95;">单分支多模式</span>

- 在 match 表达式中，可以使用 | 语法匹配多个模式，它代表 或的意思
- 在@绑定的时候也可以使用多模式，但是要使用()保证优先级

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

let num = 10;

match 1 {
    num @ (1 | 2) => {
            println!("{}", num);
    }
    _ => {}
}
```

*/
pub fn f02_01_pattern() {
    let num = Some(3);
    match num {
        Some(x @ 3..=7) => {
            println!("3..=7: {x}");
        }
        _ => (),
    }
    let num = 10;
    match num {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}

/**
# <span style="color: #a6da95;">模式解构</span>

模式解构可以用在变量绑定，match，函数参数列表等场景

## <span style="color: #a6da95;">解构结构体</span>

```rust
struct Point {
    x: i32,
    y: i32,
}
```

- 应用于变量绑定

```rust
let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p;
println!("{a} {b}");
```

- 应用于函数参数

```rust
fn test(Point { x: a, y: b }: Point) {
    println! {"{a} {b}"}
}
test(p);

let p = Point { x: 0, y: 7 };
```

- 应用于match匹配

```
match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

## <span style="color: #a6da95;">解构枚举</span>

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

- 应用于match匹配

```rust
let msg = Message::ChangeColor(0, 160, 255);

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
```

- 应用于变量绑定，需要使用let else语法处理枚举值不匹配的情况

```rust
let c = Message::ChangeColor(255, 0, 0);

let Message::ChangeColor(r, g, b) = c else {
    panic!("不匹配的枚举值");
};

println!("{r} {g} {b}");
```

## <span style="color: #a6da95;">嵌套的解构形式</span>

- 解构数组

```rust
let arr: [u16; 10] = [100; 10];
let [x, y, ..] = arr;
```

- 解构元组
```
let (x, y, .., z) = (100, 200, 0, 0, 0, 1);
```

- 可以用复杂的方式来混合、匹配和嵌套解构模式

```rust
struct Point {
     x: i32,
     y: i32,
 }

let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

- @绑定和解构同时使用

```rust
struct Point {
    x: i32,
    y: i32,
}
let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
println!("x: {}, y: {}", px, py);
println!("{:?}", p);
```

*/
#[allow(dead_code)]
pub fn f02_02_pattern_deconstruction() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("{a} {b}");

    fn test(Point { x: a, y: b }: Point) {
        println! {"{a} {b}"}
    }
    test(p);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let c = Message::ChangeColor(255, 0, 0);

    let Message::ChangeColor(r, g, b) = c else {
        panic!("不匹配的枚举值");
    };

    println!("{r} {g} {b}");

    let arr: [u16; 10] = [100; 10];
    let [x, y, ..] = arr;

    println!("{x} {y}");
}

/**
# <span style="color: #a6da95;">模式使用场景</span>

- match 分支

```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION,
}
```

- if let 分支

```
if let PATTERN = SOME_VALUE {

}
```

- while let 条件循环

```rust
// Vec是动态数组
let mut stack = Vec::new();

// 向数组尾部插入元素
stack.push(1);
stack.push(2);
stack.push(3);

// stack.pop从数组尾部弹出元素
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

- for 循环

```
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

- let 语句

```
let PATTERN = EXPRESSION;
let (x, y, z) = (1, 2, 3);
```

- 函数参数

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
```

- let和if let
    - 类似 let , for和match 都必须要求完全覆盖匹配，才能通过编译( 不可驳模式匹配 )
    - if let , while let允许匹配一种模式，而忽略其余的模式( 可驳模式匹配 )

```
// let Some(x) = some_option_value; // 编译器报错，因为let匹配的时候遗漏了None的可能性
if let Some(x) = some_option_value {
    // x 只能在if let的代码块中使用
    println!("{}", x);
}
```

- let else
    - 可驳模式匹配，且定义的变量在之后可以使用
    - else 分支中必须用发散的代码块处理（例如：break、return、panic）

```rust
// if let
if let Some(x) = some_option_value {
    println!("{}", x);
}
// x已经被丢弃，之后不可以使用

// let-else
let Some(x) = some_option_value else { return; }
// x在定义之后可以使用
println!("{}", x);
```

*/
pub fn f02_03_scence() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch06_01() {
        assert_eq!(f01_01_match(), ());
        assert_eq!(f01_02_if_let(), ());
        assert_eq!(f01_03_matches(), ());
        assert_eq!(f01_04_option(), ());
    }

    #[test]
    fn ch06_02() {
        assert_eq!(f02_01_pattern(), ());
        assert_eq!(f02_02_pattern_deconstruction(), ());
        assert_eq!(f02_03_scence(), ());
    }
}
