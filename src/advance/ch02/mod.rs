/**
## for循环和迭代器
- 迭代器用途和for循环相同，可以用来迭代一个连续的集合
- 使用迭代器只需要关系集合中的元素如何处理，无需关心如何开始、如何结束、按照什么样的索引去访问等问题
- Rust中for循环本质上就是迭代器的语法糖，通过for element in collection语法糖转换成迭代器使用
- 能被转为为迭代器的数据类型必须要实现IntoIterator特征
- 迭代器是惰性的，使用iter方法产生一个迭代器
*/
fn _ch02_01_for_iterator() {
    // 数组类型实现了IntoIterator特征可以转换为迭代器
    let arr = [1, 2, 3];
    for v in arr.into_iter() {
        println!("{}", v);
    }
    // Range类型也实现了IntoIterator特征可以转换为迭代器
    let range = 1..10;
    for i in range {
        println!("{}", i);
    }

    // 迭代器本身是惰性的，不使用的话不会发生任何事情
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for v in v1_iter {
        println!("{}", v);
    }
}

/**
## next方法
- 迭代器类型必须实现Iterator特征
- Iterator特征具有方法next，通过next方法可以获取迭代器中的元素
- next方法返回值是一个Option，它对迭代器使用是消耗性的最后就只会返回None了
- 使用next方法可以模拟for循环
*/
fn _ch02_02_next() {
    let arr = [1, 2, 3, 4, 5];
    let mut arr_iter = arr.into_iter();
    // loop循环
    loop {
        match arr_iter.next() {
            Some(value) => println!("{}", value),
            None => break,
        }
    }
    let mut arr_iter = arr.into_iter();
    // while let 循环
    while let Some(value) = arr_iter.next() {
        println!("{}", value);
    }
}

/**
## IntoIterator特征
- 实现该特征的类型可以转换为迭代器
- 迭代器类型本身也实现了这个特征
- 该特征提供三个方法转换为迭代器
  - into_iter 夺走数据所有权
  - iter 借用
  - iter_mut 可变借用
*/
fn _ch02_03_into_iterator() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 _values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出[0, 2, 3]
    println!("{:?}", values);
}

/**
## 适配器
- 消费者适配器:迭代器上的方法
  - 调用会消费掉迭代器中的元素
  - 返回其类型的值
  - 方法内部都依赖next
- 迭代器适配器:迭代器上的方法
  - 也是惰性的，返回一个新的迭代器
  - 是链式调用的关键
- 在迭代器链式调用最后必须使用一个消费者适配器来收尾，否则什么也不会发生
*/
fn _ch02_04_adapter() {
    let v1 = vec![1, 2, 3];
    // 链式调用，最后要用消费者迭代器收尾
    v1.iter()
        .map(|x| x + 1)
        .filter(|x| *x > 2)
        .for_each(|x| println!("value: {}", x));
}

/**
## 实现Iterator特征
- 为自定义类型实现Iterator特征
- 关键在于实现next方法
- Iterator特征中还有很多消费者适配器，迭代器适配器方法但是都有默认实现
*/
fn _ch02_05_impl_iter() {
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch02_01() {
        assert_eq!(_ch02_01_for_iterator(), ());
    }

    #[test]
    fn ch02_02() {
        assert_eq!(_ch02_02_next(), ());
    }

    #[test]
    fn ch02_03() {
        assert_eq!(_ch02_03_into_iterator(), ());
    }

    #[test]
    fn ch02_04() {
        assert_eq!(_ch02_04_adapter(), ());
    }

    #[test]
    fn ch02_05() {
        assert_eq!(_ch02_05_impl_iter(), ());
    }
}
