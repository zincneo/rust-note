fn _ch03_01_ownership() {
    /// ## 所有权
    /// 值在内存中保存的方式:
    /// 1. 区分变量的值保存在栈区还是堆区
    /// 2. rust没有new关键字，标量类型默认就是在栈区，自定义类型等到讲到特征的时候会说明如何判断在栈区还是堆区
    /// 3. 这里先使用标准库提供的自定义类型String来演示，String创建的变量值保存在堆区
    /// 4. 对于栈区变量使用变量绑定运算符=会发生值的拷贝copy
    /// 5. 对于堆区变量使用变量绑定运算符=会发生值的移动move
    /// 6. rust不会自动发生堆区变量复制clone，需要程序员自己触发
    /// 所有权的核心原则:
    /// 1. rust程序中所有值都有所有者
    /// 2. 一个值在同一个时刻只能存在一个所有者
    /// 3. 当所有者离开其作用域的时候对应在内存中的值会自动丢弃drop
    fn ownership() {
        // 1. copy
        let a = 10;
        let _b = a;
        // 2. move: 移动之后原来的变量不能再访问堆上的空间
        let s = String::from("test");
        let ss = s;
        println!("{ss}");
        // 3. clone
        let s = String::from("test");
        {
            let ss = s.clone();
            println!("{s} {ss}");
            // 离开时自动drop
        }
        // String这样的堆上数据类型的补充
        // 其实一个String类型的变量保存的是栈上的一个数据结构[ptr,len,capacity]，其中包含了指向String在堆上内存地址的指针
    }
    ownership();
}

fn _ch03_02_reference_borrowing() {
    /// ## rust借用
    /// - 引用reference和借用borrowing是一个概念
    /// - rust中存在可变引用和不可变引用两种引用
    /// - 引用的作用主要是在函数传递参数的时候避免所有权的转移
    /// - 同一时刻可以存在多个不可变引用或者一个可变引用
    /// - 不可变引用不可以和可变引用同时存在
    /// - 由于编译器的进步，只要一个引用之后再也不时候那么就会在最后使用的地方认为其生命周期结束
    fn reference() {
        let mut s = String::from("test");
        let _ref_s = &s;
        // 同一个时刻存在了可变和不可变引用，但是编译器智能地发现之后再也没有使用过45行定义的不可变引用，于是就在此处结束了它的生命周期
        let ref_mut_s = &mut s;
        fn take_reference(str: &mut String) {
            str.push_str(" test");
        }
        take_reference(ref_mut_s);
        println!("{ref_mut_s}");
    }
    reference();

    /// ##悬垂指针
    /// - 类似c++语言中指针可以返回函数体中的临时变量地址导致
    /// - rust的引用会检查生命周期，因此无法返回临时变量
    /// - 如此可以避免一块内存在释放之后再次访问
    fn _dangle() -> String {
        // 可以将函数体内创建出来的值所有权转移出去，但是无法返回其引用
        String::from("test")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch03_01() {
        assert_eq!(_ch03_01_ownership(), ());
    }

    #[test]
    fn ch03_02() {
        assert_eq!(_ch03_02_reference_borrowing(), ());
    }
}
