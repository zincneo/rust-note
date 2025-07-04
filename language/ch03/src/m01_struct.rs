#![allow(dead_code)]
#![allow(unused)]

/**
# 结构体定义
- Rust中使用struct关键字定义结构体
- 结构体命名遵循大驼峰规范
- 结构体内部由字段和类型组成
- 创建一个结构体对应实例的使用`StructName {key: value}`
- 初始化结构体的实例的时候每一个字段都需要初始化
- 通过.运算符可以访问结构体内部的字段
- `..`运算符可以在创建另外一个结构体实例的时候解构结构体实例
- 结构体中的字段类型使用引用类型的时候需要引入生命周期的概念，在生命周期的章节中再说
*/
pub fn f01_struct() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 字段所有权规则遵循Rust的一般原则，因此username发生了move，active和sign_in_count发生copy
    };
}

/**
# 结构体内存排布
用以下File结构体举例
```rust
 struct File {
   name: String,
   data: Vec<u8>,
 }
```
![结构体图片](/images/struct_memory.png)
- name 和 data 分别拥有底层两个 `[u8]` 数组的所有权（String 类型的底层也是 `[u8]` 数组）
- 通过 ptr 指针指向底层数组的内存地址，ptr 指针理解为 Rust 中的引用类型
- 把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段
*/
pub fn f02_memory() {
    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

/**
# 元组结构体
- Rust中结构体的字段可以没有名称
- 这种结构体定义上很像元组`struct Struct(type, type, type)`
- 访问内部字段使用.索引顺序，内部索引顺序按照定义顺序从0开始
- 和单元类型()类似，结构体也有不存在任何字段的单元结构体(Unit-like Struct)
    - 定义一个类型，但是不关心该类型的内容，只关心它的行为时，就可以使用
- 注意元组结构体和单元结构体必须加上;表示语句结束
*/
pub fn f03_tuple_struct() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);

    let red = Color(255, 0, 0);
    println!("{:?}", red);
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
