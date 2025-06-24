/*!
# 集合类型

1. 标准库提供了一些可以表示一堆元素的类型
    - 这些元素分配在堆上
    - 可以通过类型提供的方法对元素进行操作
    - 使用的最多的是Vec和HashMap
2. 本质上是实现了标准库提供的特征IntoIterator和Iterator，见迭代器章节
*/

mod m01_vector;
mod m02_hash_map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_vector() {
        assert_eq!(m01_vector::f01_vec(), ());
        assert_eq!(m01_vector::f02_sort(), ());
    }

    #[test]
    fn test02_hash_map() {
        assert_eq!(m02_hash_map::f02_hashmap(), ());
    }
}
