use std::collections::HashMap;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    if nums1.len() < nums2.len() {
        _intersect(&nums1, &nums2)
    } else {
        _intersect(&nums2, &nums1)
    }
}

fn _intersect(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();

    for num in nums1.iter().cloned() {
        map.insert(num, map.get(&num).unwrap_or(&0) + 1);
    }

    let mut out = Vec::new();
    let len1 = nums1.len();

    for num in nums2.iter().cloned() {
        let opt = map.get(&num);
        if let Some(&value) = opt {
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
mod test {
    use super::*;

    fn test<const L: usize, const M: usize, const N: usize>(
        nums1: [i32; L],
        nums2: [i32; M],
        res: [i32; N],
    ) {
        let mut nums = intersect(Vec::from(nums1), Vec::from(nums2));
        nums.sort_unstable();
        let mut res = Vec::from(res);
        res.sort_unstable();
        assert_eq!(nums, res);
    }

    #[test]
    fn case1() {
        test([1, 2, 2, 1], [2, 2], [2, 2]);
    }

    #[test]
    fn case2() {
        test([4, 9, 5], [9, 4, 9, 8, 4], [4, 9]);
    }
}
