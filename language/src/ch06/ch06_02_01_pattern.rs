/**
# Rust中的模式

## 模式内容

模式(pattern)一般由以下内容组合而来:

1. 字面值
2. 解构的数组、枚举、结构体或者元组
3. 变量
4. 通配符`_`
5. 占位符`..`
    - 占位符语义要明确，不能有编译器推导不出来的忽略位置

## 模式使用场景

1. match分支

```rust
// match VALUE { PATTERN => EXPRESSION, ... }
match 10 {
    // 1. 字面值
    10 => (),
    // 2. 变量(表示绑定到任意值)
    n => ()
}

match (10, 10, 10) => {
    // 3. 解构元组 + 字面值 + 变量
    // a, b绑定到任意值，元组第三位绑定到字面值3
    (a, b, 10) => {},
    // 4. 解构元组 + 占位符(..表示忽略那些位置的值)
    (.., 1) => {},
    // 5. 通配符_ 匹配所有剩余的情况
    _ => {}
}
```

2. if let分支

```rust
// if let PATTERN = EXPRESSION {}
let num = Some(3);
if let Some(3) = num {};
```

3. while let分支

```rust
// while let PATTERN = EXPRESSION {}
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

4. for循环
```rust
// for PATTERN in ITERATOR {}
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

5. let语句

```rust
// let PATTERN = EXPRESSION;
let x = 5; // 可以认为x也是一种模式绑定
// 解构元组模式 + 占位符忽略
let (.., a) = (1, 2, 3, 1.1);
```

6. 函数参数

```rust
// fn function(PATTERN: TYPE) {}
// 解构元组 + 占位符忽略
fn print_coordinates(&(x, y, .., z): &(i32, i32, i32, i32, i32, i32)) {
    println!("{x} {y} {z}");
}
```

## 可驳模式匹配和不可驳模式匹配

1. 不可驳回模式要求匹配所有可能性，否则编译器会报错
    - let, match

```rust
// let Some(x) = some_option_value; 编译器报错，因为let是不可驳的
// 改为可驳的
if let Some(x) = some_option_value;
```

2. 可驳模式允许匹配一种模式，忽略剩余的可能性
    - if let, while let, let else
    - let else的else块必须发散

```rust
// if let
if let Some(x) = some_option_value {
    println!("{}", x);
}

// let-else
let Some(x) = some_option_value else { return; }
println!("{}", x);
```

*/
pub fn pattern() {
    fn print_coordinates(&(x, y, .., z): &(i32, i32, i32, i32, i32, i32)) {
        println!("{x} {y} {z}");
    }
    print_coordinates(&(1, 2, 3, 4, 5, 6));
}
