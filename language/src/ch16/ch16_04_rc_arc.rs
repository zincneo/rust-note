/*!
# Rc和Arc

## [1. Rc](./fn.f04_01_rc.html)
## [2. 综合案例](./fn.f04_02_case.html)
## [3. Arc](./fn.f04_03_arc.html)
*/

use std::rc::Rc;

/**
# Rc
- Rc是Rust提供的引用计数
- 允许一个数据资源在同一时刻拥有多个所有者
```rust
let s = 1;
let a = Box::new(s); // s所有权转移给a
let b = Box::new(s); // 报错

let a = Rc::new(1);
let b = Rc::clone(&a); // 一个值多个所有者
```
- 常用的API
    - Rc::new关联方法，获取包裹的值的所有权并防止到堆上
    - Rc::clone关联方法，从另外一个Rc的引用处拷贝一份指向同一个堆上地址的指针，引用计数+1
    - Rc::strong_count关联方法，获取传入的Rc的引用计数
*/
pub fn f04_01_rc() {
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&b));
}

/**
# 综合案例

- 考虑这样的一个场景
    - 存在一个所有者类型
    - 存在一个工具类型
    - 工具类型中存储其属于哪个所有者
    - 一个工具只有一个所有者，但是多个工具可以是同一个所有者
    ```rust
    struct Owner {
        name: String,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    let owner = Rc::new(Owner {
        name: "zinc".to_string(),
    });

    let gadget_1 = Gadget {
        id: 0,
        owner: Rc::clone(&owner),
    };

    let gadget_2 = Gadget {
        id: 1,
        owner: Rc::clone(&owner),
    };
    // 提前释放掉owner
    std::mem::drop(owner);
    // 堆上的数据任然存在
    println!("Gadget {} owned by {}", gadget_1.id, gadget_1.owner.name);
    println!("Gadget {} owned by {}", gadget_2.id, gadget_2.owner.name);
    ```
*/
pub fn f04_02_case() {
    struct Owner {
        name: String,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    let owner = Rc::new(Owner {
        name: "zinc".to_string(),
    });

    let gadget_1 = Gadget {
        id: 0,
        owner: Rc::clone(&owner),
    };

    let gadget_2 = Gadget {
        id: 1,
        owner: Rc::clone(&owner),
    };
    // 提前释放掉owner
    std::mem::drop(owner);
    // 堆上的数据任然存在
    println!("Gadget {} owned by {}", gadget_1.id, gadget_1.owner.name);
    println!("Gadget {} owned by {}", gadget_2.id, gadget_2.owner.name);
}

/**
# Arc

- 用法和Rc没有区别
- Rc性能更好但不保证线程安全，Arc保证原子性性能更差适合多线程环境

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
           println!("{}", s)
        });
    }
}
```
*/
pub fn f04_03_arc() {}
