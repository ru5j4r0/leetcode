use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for num in nums {
        if !set.insert(num) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test_contains_duplicate {
    fn test(nums: Vec<i32>, res: bool) {
        assert_eq!(super::contains_duplicate(nums), res);
    }

    #[test]
    fn case1() {
        test(vec![1, 2, 3, 1], true);
    }

    #[test]
    fn case2() {
        test(vec![1, 2, 3, 4], false);
    }

    #[test]
    fn case3() {
        test(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true);
    }
}
