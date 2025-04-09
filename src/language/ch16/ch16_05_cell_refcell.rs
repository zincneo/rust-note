/*!
# Cell和RefCell
- Cell和RefCell在功能上相同，都是在拥有不可变变量的同时修改内部的值
- 内部通过`unsafe`实现
- `Cell<T>`适用T实现了Copy特征的类型，`RefCell<T>`适用于`T`不做限制

## [1. Cell](./fn.f05_01_cell.html)

## [2. RefCell](./fn.f05_02_refcell.html)

## [3. Rc和RefCell](./fn.f05_03_rc_refcell.html)
*/

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

/**
# Cell
- 提供了get、set两个方法来获取和更改值
```rust
let num = Cell::new(10);
println!("{:?}", num.get());
num.set(20);
println!("{:?}", num.get());
```
*/
pub fn f05_01_cell() {
    let num = Cell::new(10);
    println!("{:?}", num.get());
    num.set(20);
    println!("{:?}", num.get());
}

/**
# RefCell
- 由于内部可变性更多是为了解决可变、不可变引用无法共存的问题，因此比较少适用Cell，大部分情况下适用RefCell
- RefCell通过brrow、和brrow_mut两个API来获取到不可变和可变引用
```rust,should_panic
let s = RefCell::new(String::from("hello, world"));
let s1 = s.borrow();
let s2 = s.borrow_mut();
// 所有权规则下同时存在可变引用和不可变引用会直接在编译期报错
// 使用内部可变性则可以通过编译到运行期再panic
println!("{},{}", s1, s2);
```
- 所有权Vs内部可变性

| **所有权规则** | **智能指针带来的额外规则** |
|:---:|:---:|
| 一个数据只能有一个所有者 | Rc/Arc让数据可以有多个所有者 |
| 要么多个不可变借用，要么一个可变借用 | RefCell实现编译期可变、不可变引用共存 |
| 违背规则导致编译错误 | 违背规则运行期panic |

*/
pub fn f05_02_refcell() {}

/**
# Rc和RefCell
- Rc包裹RefCell的例子
- Rc提供一个值被多个变量拥有的能力
- RefCell提供内部可以被修改的能力
*/
pub fn f05_03_rc_refcell() {
    let s = Rc::new(RefCell::new("test string".to_string()));
    let s1 = s.clone();
    let s2 = s.clone();
    s2.borrow_mut().push_str(", end!");
    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}
