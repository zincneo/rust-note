fn _ch08_01_vec() {
    /// ## 动态数组
    /// - rust提供的基础类型数组的长度是固定的
    /// - 当需要动态长度的数组的时候就需要使用集合类型Vec
    /// - Vec类型支持一个泛型参数Vec<T>
    fn vec() {
        // 创建vec的两种方式
        // 1. 关联方法new
        let mut _v = Vec::<i32>::new();
        // 2. vec宏
        let mut v = vec![1, 2, 3];
        // 常用方法
        // 1. 添加元素
        v.push(1);
        // 2. 访问元素，使用下标越界会导致程序崩溃，使用get越界会返回None
        if let Some(num) = v.get(2) {
            println!("{num}")
        };
        // 3. 遍历
        for i in &mut v {
            *i += 1;
        }
        // 4. 追加多个元素
        v.extend([1, 2, 3]);
        let mut v1 = [11, 22].to_vec();
        v.append(&mut v1);
        // 5. 调整容量
        v.reserve(100);
        // 6. 查看当前容量
        println!("{}", v.capacity());
        // 7. 移除元素
        v.pop();
        v.remove(1);
        // 8. 排序
        // sort和sort_unstable方法必须要求元素实现Ord特征
        v.sort_unstable();
        v.sort();
        // 对于没有实现Ord特征的元素应该使用sort_by和sort_unstable_by方法手动指定元素比较方式
        let mut v = vec![10.0, 1.0, 2.5, 3.3];
        v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(v, vec![1.0, 2.5, 3.3, 10.0]);
    }
    vec();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch08_01() {
        assert_eq!(_ch08_01_vec(), ());
    }
}
