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
