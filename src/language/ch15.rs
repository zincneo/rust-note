/*!
# Rust深入类型

## 类型转换

1. as关键字
    - Rust中整数和浮点数类型种类较多且不能直接做运算，这时候要比较可以使用as关键字转换为对应的类型
    - 能够互相转换的类型
        - 数值类型<->数值类型
        - 内存地址(*type | *mut type)<->指针(usize)
    - 转换不具备传递性质，`e as u1 as u2`是合法的也不能认为`e as u2`合法
2. TryInto转换
    - std::convert::TryInto特征提供方法try_into来进行类型转换(同样只是针对数值类型的)
    - TryInto特征在prelude中有导出
    - 这个方法返回一个Option，会捕获大整数类型向小整数类型转换时候的溢出错误
3. [通用类型转换](./fn.f01_03_common.html)

## newtype和类型别名
1. newtype
    - 就是使用元组结构体将已有类型包裹起来
    - 目的主要是以下三条
        1. 给出更有可读性的类型名，例如表示距离的u32 `struct Meters(u32);`
        2. 为外部类型实现外部特征(特征实现必须遵循孤儿规则)
        3. 隐藏内部实现细节，Rust原生类型上有很多方法，如果要将某个类型传给用户又不想让用户调用这些方法可以使用newtype
2. 类型别名
    - 使用type关键字可以定义类型别名
    - 别名只是别名除了让可读性更好不具备newtype的其他功能
```rust
type Meters = u32;
```
## Sized和不定长类型DST

从编译何时能获知类型大小角度来说，Rust中类型可以分为两类:定长类型和不定长类型:
1. 定长类型(sized)，大小编译期已知
    1. 使用泛型参数的时候期望类型被约束为大小已知可以使用特征Sized
2. 不定长类型(unsized)，运行期才知道大小，无法直接使用，只能通过引用或者智能指针包裹间接使用
    1. 切片`[T]`没法直接使用，只能通过定长的切片引用`&[T]`进行使用
    2. `str`，作为`&str`和`String`的底层类型，无法直接使用只能间接使用
    3. 特征对象Box<dyn trait_name>，无法直接使用
## 枚举和整数
    1. 枚举值转换为整数可以直接使用as关键字
    2. 整数转换为枚举值的解决方案比较杂，繁琐一点就直接match好了
*/

use std::rc::Rc;

/**
# 通用类型转换
1. 直接通过编码函数实现类型转换
```rust
struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}
```
2. 点操作符
    1. 假设现在有一个方法foo，该方法有一个接收器(self|&self|&mut self)
    2. 对类型T的值value调用value.foo()
    3. 第一步编译器检查是否能够直接调用T::foo(value)，**这种方式称为值方法调用**
    4. 第二步(前面无法完成)，编译器尝试自动转换为引用类型是否可以调用<&T>::foo(value)和<&mut T>::foo(value)，**这种方式称为引用方法调用**
    5. 第三步，编译器尝试触发Deref特征解除引用再调用，`T: Deref<Target = U>`然后使用U类型调用方法foo，**这种方式称为解引用方法调用**
    6. 第四步，T如果是定长类型编译器会尝试转换为不定长类型调用(例如 数组[i32;2]->切片[i32])
```rust
let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([1, 2, 3]));
let first_entry = array[0];
// array[0]只是语法糖 -> array.index(0)
// 编译器检查类型Rc<Box<[i32; 3]>>是否实现Index特征，结果是否，不仅如此，&Rc<Box<[i32; 3]>> 与 &mut Rc<Box<[i32; 3]>> 也没有实现
// 编译器开始尝试自动解引用，Box<[i32; 3]>也没有实现Index特征，因此继续触发解引用为[i32; 3]实现了Index特征
```
*/
pub fn f01_03_common() {
    let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([1, 2, 3]));
    let first_entry = array[0];
    println!("{first_entry}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch15_01() {
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_address = p1 as usize; // 将p1内存地址转换为一个整数
        let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
        let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
        unsafe {
            *p2 += 1;
        }
        assert_eq!(values[1], 3);
        let a: u8 = 10;
        let b: u16 = a.try_into().unwrap();
        println!("{b}");
        assert_eq!(f01_03_common(), ());
    }

    #[test]
    fn ch15_02() {}
}
