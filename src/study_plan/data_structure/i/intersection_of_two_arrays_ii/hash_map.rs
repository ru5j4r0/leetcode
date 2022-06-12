use std::collections::HashMap;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    if nums1.len() < nums2.len() {
        _intersect(&nums1, &nums2)
    } else {
        _intersect(&nums2, &nums1)
    }
}

fn _intersect(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    for num in nums1.iter().cloned() {
        map.insert(num, map.get(&num).unwrap_or(&0) + 1);
    }

    let mut out = Vec::new();
    let len1 = nums1.len();

    for num in nums2.iter().cloned() {
        let opt = map.get(&num);
        if opt.is_some() {
            let value = *opt.unwrap();
            if value > 0 {
                out.push(num);
                if out.len() >= len1 {
                    break;
                }
            }

            map.insert(num, value - 1);
        }
    }

    out
}

#[cfg(test)]
mod test_intersection_of_two_arrays_ii {
    fn test(nums1: Vec<i32>, nums2: Vec<i32>, res: Vec<i32>) {
        let mut nums = super::intersect(nums1, nums2);
        nums.sort_unstable();
        let mut res = res;
        res.sort_unstable();
        assert_eq!(nums, res);
    }

    #[test]
    fn case1() {
        test(vec![1, 2, 2, 1], vec![2, 2], vec![2, 2]);
    }

    #[test]
    fn case2() {
        test(vec![4, 9, 5], vec![9, 4, 9, 8, 4], vec![4, 9]);
    }
}
