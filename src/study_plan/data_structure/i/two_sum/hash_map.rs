use std::collections::HashMap;

// impl Solution {
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let num = nums[i];
        let opt = map.get(&num);
        if opt.is_some() {
            return vec![*opt.unwrap(), i as i32];
        }
        map.insert(target - num, i as i32);
    }

    return vec![];
}
// }
