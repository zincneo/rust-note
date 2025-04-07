/*!
# LeetCode题解
*/
pub mod q0001;
pub mod q0002;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_testcases() {
        use q0001::solution::two_sum;
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6));
    }

    #[test]
    fn q2_testcases() {}
}
