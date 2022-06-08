use std::cmp;

// impl Solution {
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum = nums[0];
    let mut max = sum;

    for &num in nums.iter().skip(1) {
        sum = cmp::max(num, num + sum);
        if sum > max {
            max = sum;
        }
    }

    max
}
// }
