/*!
# ch04 Rust结构体和枚举

## 01 结构体

1. [结构体语法](./fn.f01_01_struct.html)
2. [结构体内存排布](./fn.f01_02_struct_memory.html)
3. [元组结构体](./fn.f01_03_tuple_struct.html)

## 02 枚举

1. [枚举语法](./fn.f02_01_enum.html)
2. [Option枚举](./fn.f02_02_option.html)
*/

/**
# 结构体语法
- Rust中使用struct关键字定义结构体
- 结构体命名遵循大驼峰规范
- 结构体内部由字段和类型组成
```rust
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
```
- 定义一个结构体对应的使用`StructName {key: value}`
- 初始化结构体的实例的时候每一个字段都需要初始化
```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```
- 通过.运算符可以访问结构体内部的字段
```rust
    println!("{}", user1.email);
```
- ..运算符可以在创建结构体实例的时候使用另外一个实例的字段的值作为值
```rust
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 字段所有权规则遵循Rust的一般原则，因此username发生了move，active和sign_in_count发生copy
    };
```
- 结构体中的字段类型使用引用类型的时候需要引入生命周期的概念，在生命周期的章节中再说
*/
#[allow(unused)]
pub fn f01_01_struct() {
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
pub fn f01_02_struct_memory() {
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
```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
```
- 访问内部字段使用.索引顺序，内部索引顺序按照定义顺序从0开始
- 和单元类型()类似，结构体也有不存在任何字段的<span style="color:#A6DA95;">单元结构体(Unit-like Struct)</span>
    - 定义一个类型，但是不关心该类型的内容，只关心它的行为时，就可以使用
- 注意元组结构体和单元结构体必须加上;表示语句结束
```rust
    struct AlwaysEqual;
    let subject = AlwaysEqual;
```
*/
pub fn f01_03_tuple_struct() {}

/**
# 枚举语法
- Rust中使用enum关键字来定义枚举
- 枚举名和枚举值遵循大驼峰规范
```rust
enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}
```
- 通过`枚举名::枚举值`来获取到枚举值
```rust
let heart = PokerSuit::Hearts;
let diamond = PokerSuit::Diamonds;
```
- Rust中的枚举很强大，枚举值可以包裹值，可以把枚举值当作结构体定义
    - 相当于同时具备了c中枚举和联合类型的能力
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
- 获取枚举值的包裹的值需要使用到模式匹配，在模式匹配章节中详细说明
- Rust枚举的一大作用就是类型同一化
    - 例如函数的参数希望传递不同类型的值的场景，就可以使用枚举做类型同一化
    - 以上面的Message举例，如果函数test想要接收以下不同类型的参数就可以使用Message
        1. `fn test()`
        2. `fn test(move: Move)`
        3. `fn test(write: String)`
        4. `fn test(color: (i32, i32, i32))`
```
fn test(message: Message) {}
```
- Rust也支持传统的枚举值映射到整型值的枚举
    - 只要所有的枚举值不包裹其他值就是传统的枚举
    - 映射到整型数字从0开始递增或使用=绑定到指定的整型值
    - 枚举值可以使用as关键字强制类型转换为整数
```rust
    enum Number {
        Zero = 0,
        One = 1,
    }

    let num = Number::Zero as i32;
```
*/
#[allow(dead_code)]
pub fn f02_01_enum() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::Write(String::from("message"));
    let m4 = Message::ChangeColor(255, 255, 0);
    println!("{:?}\n{:?}\n{:?}\n{:?}", m1, m2, m3, m4);

    enum Number {
        Zero = 0,
        One = 1,
    }

    let num = Number::Zero as i32;
    println!("{}", num);
}

/**
# Option枚举
- Rust标准库提供的一个枚举，该枚举接收一个泛型参数
- 该枚举有两个值，Some(泛型参数的类型值)和None
- 作用和c++中的空指针nullptr是类似的，这种设计可以避免空指针调用方法的null异常
- 本质上是对普通值的封装，编译器确保了在使用值的时候永远不会用到空值
```rust
#[derive]
enum Option<T> {
    Some(T),
    None
}
```
*/
pub fn f02_02_option() {
    let some_number = Some(5);
    println!("{:?}", some_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch04_01() {
        assert_eq!(f01_01_struct(), ());
        assert_eq!(f01_02_struct_memory(), ());
        assert_eq!(f01_03_tuple_struct(), ());
    }

    #[test]
    fn ch04_02() {
        assert_eq!(f02_01_enum(), ());
        assert_eq!(f02_02_option(), ());
    }
}
