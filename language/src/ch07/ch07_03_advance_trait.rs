/**
# 关联类型
- 关联类型是在特征定义代码块中，声明一个自定义类型，这样就可以在特征方法的函数签名中使用
- 也就是说关联类型起到特化泛型参数的作用，能力相对泛型参数缩小，针对具体类型实现
- 以下是标准库中迭代器特征Iterator，它有一个关联类型Item，用于代替遍历的值类型
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
- 在为具体类型实现特征的时候，需要明确定义关联类型是什么
```rust
struct Counter {
    value: u32
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}
```
- 如果不使用关联类型，而是使用泛型参数，需要注明的地方就更多
```rust
struct Counter {
    value: u32
}
// 并非标准库的Iterator定义，只是使用泛型作为演示
pub trait Iterator<Item> {
    fn next(&mut self) -> Option<Item>;
}

// 特化泛型参数
impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        // --snip--
    }
}
```
- 类型复杂的时候，使用关联类型可以提高代码可读性
```rust
pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
  type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
  fn is_null(&self) -> bool;
}
// 使用泛型参数
pub trait CacheableItem<Address>: Clone + Default + fmt::Debug + Decodable + Encodable
where
    Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash
{
  fn is_null(&self) -> bool;
}

// 使用泛型参数
trait Container<A,B> {
    fn contains(&self,a: A,b: B) -> bool;
}
fn difference<A,B,C>(container: &C) -> i32
  where
    C : Container<A,B> {
    // --snip--
}

// 使用关联类型
trait Container{
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

fn difference<C: Container>(container: &C) {}
```
- 对比特征实现时的差异
    ```rust
    // 泛型参数
    trait Test1<A, B> {
        fn test_1(&self, _a: A, _b: B) -> bool {
            return true;
        }
    }
    // 关联类型
    trait Test2 {
        type A;
        type B;
        fn test_2(&self, _a: Self::A, _b: Self::B) -> bool {
            return true;
        }
    }
    struct Test;
    // impl<A, B> Test1<A, B> for Test {} 会导致下面报错，泛型和特化类型参数只能二选一
    impl Test1<i32, i32> for Test {
        fn test_1(&self, _a: i32, _b: i32) -> bool {
            return false;
        }
    }
    // 关联类型完全等价于特化类型参数的情况,可以提高可读性
    impl Test2 for Test {
        type A = i32;
        type B = i32;
    }
    let test = Test;
    test.test_1(0, 0);
    test.test_2(0, 0);
    ```
*/
pub fn f01_type() {
    trait Test1<A, B> {
        fn test_1(&self, _a: A, _b: B) -> bool {
            return true;
        }
    }

    trait Test2 {
        type A;
        type B;
        fn test_2(&self, _a: Self::A, _b: Self::B) -> bool {
            return true;
        }
    }

    struct Test;

    // impl<A, B> Test1<A, B> for Test {} 会导致下面报错，泛型和特化类型参数只能二选一
    impl Test1<i32, i32> for Test {
        fn test_1(&self, _a: i32, _b: i32) -> bool {
            return false;
        }
    }

    // 关联类型完全等价于特化类型参数的情况,可以提高可读性
    impl Test2 for Test {
        type A = i32;
        type B = i32;
    }

    let test = Test;
    test.test_1(0, 0);
    test.test_2(0, 0);
}

/**
# 调用同名方法

- 类型和特征上具有同名方法的情况
- Rust中没有重载同名方法的做法，所以存在同名方法没法根据传入参数类型的差异决定调用哪一个
- 首先会调用类型本身的方法
- 类型上没有该名字的方法才会使用特征上的方法
- 当存在类型本身的方法和特征方法同名时，需要使用`Trait::method(&self_obj)`来进行调用
- 关联方法重名的时候使用完全限定语法`<Type as Trait>::method()`来进行调用

```rust
trait Pilot {
    fn fly(&self);
    fn test() {}
}

trait Wizard {
    fn fly(&self, _a: i32);
    fn test() {}
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self, _a: i32) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
    fn test() {}
}
let human = Human;
human.fly();
Wizard::fly(&human, 0);
Pilot::fly(&human);
Human::test();
<Human as Wizard>::test();
<Human as Pilot>::test();
```
*/
pub fn f02_same_name() {
    trait Pilot {
        fn fly(&self);
        fn test() {}
    }

    trait Wizard {
        fn fly(&self, _a: i32);
        fn test() {}
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self, _a: i32) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
        fn test() {}
    }

    let human = Human;
    human.fly();
    Wizard::fly(&human, 0);
    Pilot::fly(&human);
    Human::test();
    <Human as Wizard>::test();
    <Human as Pilot>::test();
}

/**
# 默认泛型参数
当使用泛型类型参数时，可以为其指定一个默认的具体类型，例如标准库中的 std::ops::Add 特征：
默认类型参数主要用于两个方面：
- 减少实现的样板代码，例如标准库的加法特征
    ```rust
    trait Add<RHS=Self> {
        type Output;

        fn add(self, rhs: RHS) -> Self::Output;
    }
    ```
- 扩展类型但是无需大幅修改现有的代码
- 有多个泛型参数的时候，具有默认泛型参数的必须在最后出现
*/
pub fn f03_default_generic() {}
