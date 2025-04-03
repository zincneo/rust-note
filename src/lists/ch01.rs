/*!
# 单向链表

## 01 数据布局

- [01 无法编译的定义](./fn.f01_01_cannot_compiler.html)
- [02 使用Box智能指针](./fn.f01_02_use_box.html)
- [03 优化结构消除junk](./fn.f01_03_list_link_node.html)

## [02 基本操作](./fn.f02_basic_method.html)

## [03 最终实现](./fn.f03_impl.html)
*/

use std::mem;

/**
1. 编译器会报错的链表
```rust
pub enum List {
    Empty,
    Elem(i32, List),
}
```
编译器要求栈上的数据类型在编译期大小是已知的，枚举的枚举值Elem包含了枚举List本身，嵌套本身的解构形成了无限嵌套编译器推不出来大小
*/
pub fn f01_01_cannot_compiler() {}

/**
2. 使用Box智能指针
```rust
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
```
Box指向一个堆上的数据，但Box本身在栈上大小固定，这下可以编译了
缺点是如果创建一个拥有两个节点的链表就会造成如下的结果
```
[] = Stack
() = Heap
[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
```
- 最后一个节点分配在了堆上，但是它看上去根本不像一个 Node
- 第一个 Node 是存储在栈上的，结果一家子不能整整齐齐的待在堆上了
- 所谓的junk就是指不必要的内存空间，这里在最后一个节点是Empty，但这个Empty却要占用和Elem相同大小的内存空间，这块浪费的空间就称为junk
*/
pub fn f01_02_use_box() {}

/**
3. 让所有节点都在堆上
```
[ptr] -> (Elem A, ptr) -> (Elem B, *nullptr*)
```
无条件的在堆上创建所有节点，最大的区别就是这里不再有**junk**

关于Rust中的内存对齐，以下述枚举举例:

```rust
enum Foo {
   D1(u8),
   D2(u16),
   D3(u32),
   D4(u64)
}
```

上述枚举值`Foo::D1(99)`会占用8个字节而不是u8的一个字节，这是因为所有成员都会和枚举类型的内存大小对齐，可以说这样只需要u8大小却要分配8字节的行为就造成了junk(内存浪费)
对于之前的List结构来说造成junk的就是Empty这个枚举值，不需要占用内存空间但是还是占据了和Elem(i32, Box<List>)系统的空间

在处理内存对齐的时候编译器会对空指针编译优化的特例
```rust
enum Foo {
    A,
    B(ContainsANonNullPtr),
}
```
在这里 null 指针的优化就开始介入了，它会消除枚举成员 A 占用的额外空间，原因在于编译器可以直接将 A 优化成 0，而 B 则不行，因为它包含了非 null 指针。这样一来，编译器就无需给 A 打 tag 进行识别了，而是直接通过 0 就能识别出这是 A 成员，非 0 的自然就是 B 成员

事实上，编译器还会对枚举做一些其他优化，但是 null 指针优化是其中最重要的一条

解决以上问题并且让所有节点都在堆上，可以将Node和List分离

```rust

struct Node {
elem: i32,
next: List
}

pub enum List {
    // Empty享受空指针优化，Empty不占用内存空间，不会造成junk
    Empty,
    // 这里会导致编译器报错，因为pub enum会要求所有成员pub，但是Node没有声明为pub
    More(Box<Node>)
}
```

- List 的尾部不会再分配多余的 junk 值
- List 枚举的形式可以享受 null 指针优化
- 所有的元素都拥有统一的内存分配
- 所有的元素都拥有统一的内存分配

```rust
// 使用结构体包裹头指针，struct不要求所有成员pub
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
```
*/
pub fn f01_03_list_link_node() {}

/**
# 链表的基本操作

- new
```rust
impl List {
    fn new() -> Self {
        Self { head: Link::Empty }
    }
}
```
- push
    - 这里实现头插法
    - 注意所有权规则
    - 标准库提供的函数std::mem::replace允许将一个可变引用的值偷出来，放进去另外一个值
```rust
impl List {
    fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            // 1. next: self.head // 发生所有权转移导致编译器报错
            // 2. next: self.head.clone() // 产生一个深拷贝，内存消耗大
            next: std::mem::replace(&mut self.head, List::Empty) // 标准库函数，偷走原来的值放进去Empty
        };
        self.head = new_node;
    }
}
```
- pop
    - 注意所有权规则
```rust
impl List {
    fn pop(&mut self) -> Option<i32> {
        // 1. match self.head // 发生所有权转移导致编译器报错
        // 2. match &self.head // 引用形式，移除内容不适用
        // 标准库函数偷出来原来的值使用
        match std::mem::replace(&mut self.head, List::Empty) {
            List::Empty => None,
            List::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
```
*/
#[allow(unused)]
#[allow(dead_code)]
pub fn f02_basic_method() {
    pub struct List {
        head: Link,
    }

    #[derive(Clone)]
    enum Link {
        Empty,
        More(Box<Node>),
    }

    #[derive(Clone)]
    struct Node {
        elem: i32,
        next: Link,
    }

    impl List {
        pub fn new() -> Self {
            List { head: Link::Empty }
        }
        pub fn push(&mut self, elem: i32) {
            let new_node = Box::new(Node {
                elem,
                next: std::mem::replace(&mut self.head, Link::Empty),
            });
            self.head = Link::More(new_node);
        }
        pub fn pop(&mut self) -> Option<i32> {
            match std::mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(node) => {
                    self.head = node.next;
                    Some(node.elem)
                }
            }
        }
    }
}

/**
# 最终实现
- 注意内存释放的问题
- 在Rust中变量离开作用域的时候会自动调用Drop特征中的方法销毁数据
- 无需手动为自定义类型实现 Drop 特征，原因是 Rust 自动为几乎所有类型都实现了 Drop，只要结构体的所有字段都实现了 Drop，那结构体也会自动实现 Drop
- 但是默认实现可能不够好
    - 有这样一个链表`List -> A -> B -> C`
    - 默认实现会在List自动Drop之后在递归处理`A B C`
    - 如果这个链表非常长那么就会导致stack overflow
    - 这看起像是尾递归
        - 尾递归是一种特殊的递归形式，在函数调用自身时，递归调用是函数体中最后执行的操作，并且其返回值不再参与任何其他计算。这种递归形式可以通过编译器优化，减少栈空间的使用，从而提高程序的运行效率
        - 靠谱的编程语言编译器会优化尾递归避免函数调用栈stack overflow
        - 然而以上实现的链表在drop过程中并非尾递归
        - 以下注释可以看出为 Box<Node> 实现的 drop 方法中，在 self.ptr.drop 后调用的 deallocate 会导致非尾递归的情况发生
```rust
// 模拟编译器实现的情况
impl Drop for List {
    fn drop(&mut self) {
        // NOTE: 在 Rust 代码中，我们不能显式的调用 `drop` 方法，只能调用 std::mem::drop 函数
        // 这里只是在模拟编译器!
        self.head.drop(); // 尾递归 - good!
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        match *self {
            Link::Empty => {} // Done!
            Link::More(ref mut boxed_node) => {
                boxed_node.drop(); // 尾递归 - good!
            }
        }
    }
}

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // 糟糕，这里不是尾递归!
        deallocate(self.ptr); // 不是尾递归的原因是在 `drop` 后，还有额外的操作
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}

```
- 手动为List类型实现Drop特征
```rust
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            // 偷出来next
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
        }
    }
}
```
*/
pub fn f03_impl() {
    struct List {
        head: Link,
    }

    impl Drop for List {
        fn drop(&mut self) {
            let mut cur_link = mem::replace(&mut self.head, Link::Empty);
            while let Link::More(mut boxed_node) = cur_link {
                // next偷出来
                cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
                // boxed_node在这里超出作用域被drop
                // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
                // 因此这里并不会有无边界的递归发生
            }
        }
    }

    #[derive(Clone)]
    enum Link {
        Empty,
        More(Box<Node>),
    }

    #[derive(Clone)]
    struct Node {
        elem: i32,
        next: Link,
    }

    impl List {
        fn new() -> Self {
            List { head: Link::Empty }
        }
        fn push(&mut self, elem: i32) {
            let new_node = Box::new(Node {
                elem,
                next: std::mem::replace(&mut self.head, Link::Empty),
            });
            self.head = Link::More(new_node);
        }
        fn pop(&mut self) -> Option<i32> {
            match std::mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(node) => {
                    self.head = node.next;
                    Some(node.elem)
                }
            }
        }
    }

    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}
