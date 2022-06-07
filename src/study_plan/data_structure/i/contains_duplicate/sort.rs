// impl Solution {
pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort_unstable();
    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }
    return false;
}
// }
