pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort_unstable();

    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test_contains_duplicate {
    fn test<const N: usize>(nums: [i32; N], res: bool) {
        assert_eq!(super::contains_duplicate(Vec::from(nums)), res);
    }

    #[test]
    fn case1() {
        test([1, 2, 3, 1], true);
    }

    #[test]
    fn case2() {
        test([1, 2, 3, 4], false);
    }

    #[test]
    fn case3() {
        test([1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true);
    }
}
