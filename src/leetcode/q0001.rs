/*!
# Two Sum
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

**Example 1:**

```
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

**Example 2:**

```
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

**Constraints:**

- 2 <= nums.length <= 104
- -109 <= nums[i] <= 109
- -109 <= target <= 109
- Only one valid answer exists.

*/

/**
# Solution

1. 两层for循环，时间复杂度log(n<sup>2</sup>)
2. 使用标准库的哈希表，迭代数组的时候检查哈希表中是否存在`target - currentVal`

*/
pub mod solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::<i32, i32>::new();
        for (idx, &val) in nums.iter().enumerate() {
            if let Some(&preidx) = map.get(&(target - val)) {
                return vec![preidx, idx as i32];
            } else {
                map.insert(val, idx as i32);
            }
        }
        panic!("incorrect testcase");
    }
}
