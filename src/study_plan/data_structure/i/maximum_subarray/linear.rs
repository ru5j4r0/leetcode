use std::cmp;

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

#[cfg(test)]
mod test_maximum_subarray {
    fn test<const N: usize>(nums: [i32; N], res: i32) {
        assert_eq!(super::max_sub_array(Vec::from(nums)), res);
    }

    #[test]
    fn case1() {
        test([-2, 1, -3, 4, -1, 2, 1, -5, 4], 6);
    }

    #[test]
    fn case2() {
        test([1], 1);
    }

    #[test]
    fn case3() {
        test([5, 4, -1, 7, 8], 23);
    }
}
