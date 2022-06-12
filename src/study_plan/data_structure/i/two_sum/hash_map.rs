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
    fn test(nums: Vec<i32>, target: i32, res: Vec<i32>) {
        let mut nums = super::two_sum(nums, target);
        let mut res = res;
        nums.sort_unstable();
        res.sort_unstable();
        assert_eq!(nums, res);
    }

    #[test]
    fn case1() {
        test(vec![2, 7, 11, 15], 9, vec![0, 1]);
    }

    #[test]
    fn case2() {
        test(vec![3, 2, 4], 6, vec![1, 2]);
    }

    #[test]
    fn case3() {
        test(vec![3, 3], 6, vec![0, 1]);
    }
}
