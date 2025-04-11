/**
# 特征定义和实现

- 定义特征使用`trait TraitName {}`
```rust
trait Summary {
    fn summarize(&self) -> String;
}
```
- 实现特征使用`impl TraitName for StructOrEnumName {}`
```rust
struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}
```
- 在定义特征时可以提供方法的默认实现
    - 在实现特征的时候可以使用默认的也可以选择重载特征中的方法
```
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
// 使用默认实现
impl Summary for Post {}
// 重载方法
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
```

- 为类型A实现特征T，那么A和T至少有一个在当前作用域中定义
    - 该规则称为**孤儿规则**，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码

*/
#[allow(unused)]
#[allow(dead_code)]
pub fn f02_01_trait() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub struct Post {
        pub title: String,   // 标题
        pub author: String,  // 作者
        pub content: String, // 内容
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }
}

/**
# 使用特征
- 作为函数的参数
```rust
trait Test {
    fn test(&self) {
        println!("test");
    }
}
struct A;
struct B;
impl Test for A {}
impl Test for B {}
fn test(obj: &impl Test) {
    obj.test();
}
let (a, b) = (A, B);
test(&a);
test(&b);
```

- 特征约束
    - impl TraitName本质上就是特征约束的语法糖
    - 特征约束语法`<T: TraitName>`
    - `: Trait`不仅仅可以约束类型，也可以约束特征(表示在实现约束特征的情况下才能实现该特征)

```rust
fn test(obj: &impl Test) {}
// 等价于使用特征约束
fn test<T: Test>(obj: &T) {}
```

- 多重约束
    - 可以使用+运算符表示被多个特征约束
    - 语法糖形式`fn notify(item: &(impl Summary + Display)) {}`
    - 特征约束形式`fn notify<T: Summary + Display>(item: &T) {}`

- where特征约束
    - 特征约束比较复杂的时候不想让函数签名过于复杂可以使用where

```rust
fn some_function_v1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// 等价于
fn some_function_v2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

- 使用特征约束可以有条件地实现方法

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 任意泛型参数的Pair都有new方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// 泛型参数实现约束特征的时候才可以使用的方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

- 函数返回中的impl TraitName
    - 注意这种方式编译器只能推导出确定的唯一类型，并不能真的返回不同类型的值
    - 如果要返回不同类型的值则需要使用特征对象
    - 用途是如果返回值类型过于复杂不好声明(例如闭包和迭代器)可以使用

```rust
// 只能推导出确定的唯一类型，返回值类型容易写的就没必要这么用了
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}
```

- 调用方法需要引入特征
    - 如果要使用某一特征的方法，那么需要在当前作用域引入对应的特征

```rust
use std::convert::TryInto;

fn main() {
  let a: i32 = 10;
  let b: u16 = 100;

  // try_into方法属于TryInto特征，要引入特征才可以使用
  let b_ = b.try_into()
            .unwrap();

  if a < b_ {
    println!("Ten is less than one hundred.");
  }
}
```

- 通过实现特征来重载运算符

```rust
use std::ops::Add;

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}
```
*/
pub fn f02_02_use_trait() {
    trait Test {
        fn test(&self) {
            println!("test");
        }
    }
    struct A;
    struct B;
    impl Test for A {}
    impl Test for B {}
    fn test(obj: &impl Test) {
        obj.test();
    }
    let (a, b) = (A, B);
    test(&a);
    test(&b);

    use std::ops::Add;

    #[derive(Debug)]
    struct Point<T>
    where
        T: Add<T, Output = T>,
    {
        x: T,
        y: T,
    }
    impl<T> Add for Point<T>
    where
        T: Add<T, Output = T>,
    {
        type Output = Point<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", p1 + p2);

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", p3 + p4);
}
