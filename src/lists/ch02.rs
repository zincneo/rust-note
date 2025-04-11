/*!
# 优化单向链表

## 优化类型定义

### 使用Option

```rust
struct List {
    head: Link,
}
// 使用Option取代Empty
type Link = Option<Box<Node>>;
struct Node {
    elem: i32,
    next: Link
}
```

- Option提供了take方法，功能可以替代std::mem::replace
    - 偷走值放入None
    - 见方法push的注释
- Option可以使用map方法替代`match option { None => None, Some(x) => Some(y) }`
    - 见方法pop的注释

### 使用泛型

```rust
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
```

## 实现迭代器转换

- 集合类型可以通过Iterator特征进行迭代

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

- 每个集合类型应该实现三种迭代器类型
    - IntoIter - T
    - IterMut - &mut T
    - Iter - &T

```rust
// IntoIter实现
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}
```

```rust
// Iter实现
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// 这里不需要生命周期，List类型本身不包含引用类型
impl<T> List<T> {
    // 这里的生命周期注明是为了返回值里面作为Iter返回值类型的生命周期标识符参数
    // 编译器可以自动推断返回值的类型要跟着self，但是类型推导这个标识符推导不出来，要手动注明
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            // 实现上要注意的坑
            // self.head.map() // map拿到的是self，这里iter方法取到的是&self，因此要先调用as_ref，让类型变为&Box<Node>，再调用map，拿到的就是&self
            // &Box<Node> 要返回&Node类型，没有调用方法因此不会触发Deref特征，需要手动解引用&**node
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // 让Iter结构体next字段Option<&Node>保存的值为下一个节点的引用
            // 闭包当中没法通过类型推导自动Deref，但是map是一个泛型函数，可以通过调用限定语法来要求其推导对应类型来触发Deref
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    // next方法可以脱糖
    // 可以看出next方法的输入生命周期和输出生命周期没有关系，因此可以无条件一直调用next方法
    fn next<'b>(&'b mut self) -> Option<&'a T> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &node.elem
        })
    }
}
```

```rust
// IterMut实现
pub struct IterMut<'a, T> {
    // 保存节点的可变引用
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    // 生命周期脱糖，IterMut里面的生命周期标识符来&mut self，可以用'_让编译器自己推导
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
```
*/
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    /**
    # Option优化后的push
    - 使用Option的take方法替代掉std::mem::replace
    - take方法会直接获取引用的值，并填入None
    ```rust
    // 优化前
    pub fn push(&mut self, elem: T) {
        let next = std::mem::replace(self.head, List::Empty);
        let new_node = Box::new(Node) {
            elem,
            next,
        };
        self.head = List::More(new_node);
    }
    // 优化后
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    ```
    */
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /**
    # 使用Option优化后的pop
    - take方法取代了原来的std::mem::replace
    - 不需要`match self.head { List::Empty => ..., List::More(node) => ...}`，Option实现了Iterator特征直接使用map方法
    ```rust
    // 优化前
    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(self.head, List::Empty) {
            None => None,
            List::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    // 优化后
    // map是迭代器的消费者适配器
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    ```
    */
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    /**
    # peek方法
    1. map方法通过self获取值会导致所有权转移，编译失败
        - `self.head.map(|node| &node.elem)` 这里node是局部变量，不可以使用凭空产生的生命周期
    2. 使用as_ref方法，先获取到引用再map，同时注意返回引用，这里node.elem会触发自动解引用
        - `self.head.as_ref().map(|node| &node.elem)`由于拿到的是引用类型，因此返回引用的生命周期来源是链表self不会导致报错
    */
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /**
    # peek_mut方法
    1. 注意模式匹配问题
        - `self.head.as_mut().map()` 这里map作为闭包接收到的参数类型是`&mut Box<Node<T>>`
        - 所以如果pattern给的是`self.head.as_mut().map(|&mut node|)` 则node匹配出来的类型是`Box<Node<T>>`，也就不能返回可变引用了，因为`Box<Node<T>>`变成了闭包内部的局部变量，不能凭空产生生命周期
    2. 正确的写法`self.head.as_mut().map(|node| &mut node.elem)`
    */
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// 这里不需要生命周期，List类型本身不包含引用类型
impl<T> List<T> {
    // 这里的生命周期注明是为了返回值里面作为Iter返回值类型的生命周期标识符参数
    // 编译器可以自动推断返回值的类型要跟着self，但是类型推导这个标识符推导不出来，要手动注明
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            // 实现上要注意的坑
            // self.head.map() // map拿到的是self，这里iter方法取到的是&self，因此要先调用as_ref，让类型变为&Box<Node>，再调用map，拿到的就是&self
            // &Box<Node> 要返回&Node类型，没有调用方法因此不会触发Deref特征，需要手动解引用&**node
            // next: self.head.as_ref().map(|node| &**node),
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // 让Iter结构体next字段Option<&Node>保存的值为下一个节点的引用
            // 闭包当中没法通过类型推导自动Deref，但是map是一个泛型函数，可以通过调用限定语法来要求其推导对应类型来触发Deref
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    // 保存节点的可变引用
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    // 生命周期脱糖，IterMut里面的生命周期标识符来&mut self，可以用'_让编译器自己推导
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
