pub mod q1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_testcases() {
        assert_eq!(vec![0, 1], q1::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], q1::two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], q1::two_sum(vec![3, 3], 6));
    }
}
