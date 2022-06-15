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
mod test {
    use super::*;

    fn test<const N: usize>(nums: [i32; N], res: bool) {
        assert_eq!(contains_duplicate(Vec::from(nums)), res);
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
