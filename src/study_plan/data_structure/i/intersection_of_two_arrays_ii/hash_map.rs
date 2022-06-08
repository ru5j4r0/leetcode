use std::collections::HashMap;

// impl Solution {
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    if nums1.len() < nums2.len() {
        _intersect(&nums1, &nums2)
    } else {
        _intersect(&nums2, &nums1)
    }
}
// }

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
