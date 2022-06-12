use std::collections::HashMap;

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

    vec![]
}

#[cfg(test)]
mod test_two_sum {
    fn test<const M: usize, const N: usize>(nums: [i32; M], target: i32, res: [i32; N]) {
        let mut nums = super::two_sum(Vec::from(nums), target);
        let mut res = Vec::from(res);
        nums.sort_unstable();
        res.sort_unstable();
        assert_eq!(nums, res);
    }

    #[test]
    fn case1() {
        test([2, 7, 11, 15], 9, [0, 1]);
    }

    #[test]
    fn case2() {
        test([3, 2, 4], 6, [1, 2]);
    }

    #[test]
    fn case3() {
        test([3, 3], 6, [0, 1]);
    }
}
