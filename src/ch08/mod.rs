use std::collections::HashMap;

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

fn _ch08_02_string() {
    /// ## 字符串
    /// - 语言本身提供的字符串str
    /// - String则是标准库提供的集合类型，元素是UTF-8编码的Unicode字符，单个字符长度1-4字节
    fn string() {
        // 语言提供的字面值是str类型，这个类型是一个切片类型，变量得到的是对硬编码的str类型的引用
        let _s = "str";
        // 1. 新建字符串
        let _s = String::new();
        let _s = String::from("str");
        // 2. 字符串遍历方法，注意String类型不支持索引
        let s = String::from("Здравствуйте");
        for c in s.chars() {
            println!("{c}");
        }
        // 3. 字符串切片:最好不要对包含ACNI之外字符的字符串使用，切不到完整的字符位置会导致程序崩溃
        let s = String::from("abcd");
        let s = &s[0..2];
        println!("{s}");
    }
    string();
}

fn _ch08_03_map() {
    /// ## 哈希表
    /// - 标准库提供的HashMap类型
    /// - key->value HashMap<K, V>
    fn map() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 80);
        scores.insert(String::from("Yello"), 30);
        for (k, v) in scores.iter() {
            println!("{k} : {v}");
        }
    }
    map();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch08_01() {
        assert_eq!(_ch08_01_vec(), ());
    }

    #[test]
    fn ch08_02() {
        assert_eq!(_ch08_02_string(), ());
    }

    #[test]
    fn ch08_03() {
        assert_eq!(_ch08_03_map(), ());
    }
}
