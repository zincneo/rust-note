/*!
# Reverse Integer

Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2<sup>31</sup>, 2<sup>31</sup> - 1], then return 0.

**Assume the environment does not allow you to store 64-bit integers (signed or unsigned).**

**Example 1:**

```
Input: x = 123
Output: 321
```

**Example 2:**

```
Input: x = -123
Output: -321
```

**Constraints:**

-2<sup>31</sup> <= x <= 2<sup>31</sup> - 1
*/

/**
# Solution

- 反转可能会造成溢出，因此每次迭代的时候判断一下是否大于最大32位整数/10和小于最小32位整数/10

*/
pub mod solution {
    pub fn reverse(x: i32) -> i32 {
        let (max, min) = (i32::MAX / 10, i32::MIN / 10);
        let mut x = x;
        let mut rev = 0;
        while x != 0 {
            if rev > max || rev < min {
                return 0;
            }
            rev *= 10;
            rev = rev + (x % 10);
            x /= 10;
        }
        rev
    }
}

#[test]
pub fn test() {
    use solution::reverse;
    assert_eq!(123, reverse(321));
    assert_eq!(-123, reverse(-321));
}
