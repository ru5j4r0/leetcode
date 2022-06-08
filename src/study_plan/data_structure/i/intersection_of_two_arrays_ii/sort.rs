// impl Solution {
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1.clone();
    nums1.sort_unstable();

    let mut nums2 = nums2.clone();
    nums2.sort_unstable();

    let mut out = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;
    let len1 = nums1.len();
    let len2 = nums2.len();

    for _ in 0..len1 + len2 {
        let num1 = nums1[i1];
        let num2 = nums2[i2];

        if num1 == num2 {
            out.push(num1);

            i1 += 1;
            i2 += 1;
            if i1 >= len1 || i2 >= len2 {
                break;
            }
        } else if num1 < num2 {
            i1 += 1;
            if i1 >= len1 {
                break;
            }
        } else {
            i2 += 1;
            if i2 >= len2 {
                break;
            }
        }
    }

    out
}
// }
