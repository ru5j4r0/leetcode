use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut sets = HashSet::new();
        for num in nums {
            if !sets.insert(num) {
                return true;
            }
        }
        return false;
    }
}
