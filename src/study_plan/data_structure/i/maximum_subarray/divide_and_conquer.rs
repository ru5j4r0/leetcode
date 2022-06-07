use std::cmp;

// impl Solution {
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    _max_sub_array(&nums, 0, nums.len() - 1)
}
// }

fn max_3(x: i32, y: i32, z: i32) -> i32 {
    cmp::max(x, cmp::max(y, z))
}

fn _max_sub_array(nums: &Vec<i32>, left_index: usize, right_index: usize) -> i32 {
    let mid_index = (left_index + right_index) / 2;
    if left_index == right_index {
        return nums[left_index];
    } else if left_index == mid_index {
        let left_sum = nums[left_index];
        let right_sum = nums[right_index];
        let sum = left_sum + right_sum;
        return max_3(left_sum, right_sum, sum);
    }

    max_3(
        mid_sum(nums, left_index, mid_index, right_index),
        _max_sub_array(nums, left_index, mid_index - 1),
        _max_sub_array(nums, mid_index + 1, right_index),
    )
}

fn mid_sum(nums: &Vec<i32>, left_index: usize, mid_index: usize, right_index: usize) -> i32 {
    max_sum(nums, (left_index..mid_index).rev(), nums[mid_index])
        + max_sum(nums, mid_index + 1..=right_index, 0)
}

fn max_sum<T>(nums: &Vec<i32>, iter: T, init: i32) -> i32
where
    T: IntoIterator<Item = usize>,
{
    let mut sum = init;
    let mut max = init;
    for i in iter {
        sum += nums[i];
        if sum > max {
            max = sum;
        }
    }
    max
}
