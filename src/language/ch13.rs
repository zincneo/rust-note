/*!
# Rust格式化输出

1. 常用输出宏
    - print! 将格式化文本输出到标准输出，不带换行符
    - println! 同上，但是在行的末尾添加换行符
    - format! 将格式化文本输出到 String 字符串
    - eprint! 将格式化文本输出到标准错误输出，不带换行符
    - eprintln! 同上，但是在行的末尾添加换行符
```rust
println!("hello rust");
```
2. 输出宏中的占位符
    - Rust中使用`{}`作为格式化字符串中的占位符
    - `{:?}`也是占位符，`{:#?}`是更美观的`{:?}`
    - 区别是`{}`要求实现std::fmt::Display特征，`{:?}`要求实现std::fmt::Debug特征
```rust
let i = 3.1415926;
let s = String::from("hello");
let v = vec![1, 2, 3];
println!("{} {:?} {:?}", i, s, v);
```

3. 位置参数
    - 格式化字符串后的参数位置索引技术从0开始
    - `{}`占位符中不填入索引则从0开始递增

```rust
println!("{1}{0}", 1, 2); // =>"21"
println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
println!("{1}{}{0}{}", 1, 2); // => 2112
```

4. 具名参数
    - 带名称的参数必须放在不带名称参数的后面

```rust
println!("{argument}", argument = "test"); // => "test"
println!("{name} {}", 1, name = 2); // => "2 1"
// println!("{abc} {1}", abc = "def", 2); // 报错
```

5. 格式化参数
    1. 填充`{:[填充字符][对齐方式][宽度]}`
        1. 对齐方式: `<`左对齐(右边填充) `^`居中对齐 `>`右对齐(左边填充)
        2. 宽度: 索引和具名参数后需要加上$符号
        3. 字符串默认填充字符是空字符，数字默认填充字符是0
    ```rust
    //-----------------------------------
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 为"x"后填充@字符，补齐宽度5
    println!("Hello {:@>5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------

    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);
    ```
    2. 精度
    ```rust
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);
    ```
    3. 进制
        - `#b` `#o` `#x` `#X` `x`
    ```rust
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);
    ```
    4. 指数
    ```rust
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9
    ```
    5. 指针地址
    ```rust
    let v= vec![1, 2, 3];
    println!("{:p}", v.as_ptr()) // => 0x600002324050
    ```
    6. 转义
        1. 使用`\`作为转义字符
        2. 对于`{`使用`{{`来转义，`}`使用`}}`

6. 格式化字符串中捕获环境中的值
    - 写法和具名参数一致，在参数列表中没有的时候捕获环境中的值
```rust
let p = 5;
println!("Hello, {}!", p);                // implicit position
println!("Hello, {0}!", p);               // explicit index
println!("Hello, {person}!", person = p);
// 捕获环境中的值
println!("Hello, {p}");
```

7. 调试输出常用宏(以下几个宏零运行时开销，编译期就替换了)
    1. `line!`返回当前行号
    2. `file!`返回当前文件路径
    3. `module_path!`返回模块路径
```rust
println!(
    "位置: {} (行号: {}) - 模块: {}",
    file!(),
    line!(),
    module_path!()
);
```
*/
