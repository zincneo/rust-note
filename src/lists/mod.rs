/*!
# Rust实现链表

## ch00 Rust中到底需不需要使用链表

- 从数据结构的定义上来说，链表就是保存了指向下一个同类型元素数据的结构
- 从语言实现的角度上来说，链表就是堆上的一些元素，这些元素包含了指针，指向下一个元素或空位置
- 函数式语言中定义的链表
    - `List a = Empty | Elem a (List a)`
    - a要么为空要么是一个元素后面再跟着一个链表
- 链表的主要用途
    - 对列表进行大量的分割和合并操作
    - 无锁并发
    - 要实现内核或者嵌入式服务
    - 纯函数式的语言，受限于语法以及缺少可变性，只能使用链表解决问题
- Rust中大部分情况下不如使用Vec或者其他集合类型解决问题，除非以下情况
    - Leetcode做题
    - 构建非常适合链表的并发数据结构

## [ch01 单向链表](./ch01/index.html)
## [ch02 优化单向链表](./ch02/index.html)
*/
pub mod ch01;
pub mod ch02;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch01_01() {
        assert_eq!(ch01::f01_01_cannot_compiler(), ());
        assert_eq!(ch01::f01_02_use_box(), ());
        assert_eq!(ch01::f01_03_list_link_node(), ());
    }

    #[test]
    fn ch01_02() {
        assert_eq!(ch01::f02_basic_method(), ());
    }

    #[test]
    fn ch01_03() {
        assert_eq!(ch01::f03_impl(), ());
    }

    #[test]
    fn ch02_02() {
        use ch02::List;

        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));

        println!("TEST ---- into_iter");

        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        println!("TEST ---- iter");

        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        println!("TEST ---- iter_mut");

        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
