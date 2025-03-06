fn _ch09_01_unrecoverable() {
    /// ## 不可恢复的崩溃
    /// - panic宏可以主动让当前线程崩溃
    /// - rust不处理错误的时候会让程序崩溃
    /// - 数组越界访问会导致程序崩溃
    fn unrecoverable() {
        // 1. panic宏
        // panic!("crash and burn");
        // 2. 数组越界访问
        let v = vec![1, 2, 3];
        v[99];
    }
    unrecoverable();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch09_01() {
        assert_eq!(_ch09_01_unrecoverable(), ());
    }
}
