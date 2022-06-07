// impl Solution {
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return;
    } else if m == 0 {
        nums1.clone_from(nums2);
        return;
    }

    let m = m as usize;
    let n = n as usize;
    let mut mi = m - 1;
    let mut ni = n - 1;
    for i in (0..(m + n)).rev() {
        let num1 = nums1[mi];
        let num2 = nums2[ni];
        if num1 > num2 {
            nums1[i] = num1;
            if mi == 0 {
                nums1[..i].clone_from_slice(&nums2[..=ni]);
                return;
            }
            mi -= 1;
        } else {
            nums1[i] = num2;
            if ni == 0 {
                return;
            }
            ni -= 1;
        };
    }
}
// }
