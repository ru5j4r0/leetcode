use std::cmp::Ordering::*;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    nums1.sort_unstable();

    let mut nums2 = nums2;
    nums2.sort_unstable();

    let mut out = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;
    let len1 = nums1.len();
    let len2 = nums2.len();

    for _ in 0..len1 + len2 {
        let num1 = nums1[i1];
        let num2 = nums2[i2];

        match num1.cmp(&num2) {
            Less => {
                i1 += 1;
                if i1 >= len1 {
                    break;
                }
            }
            Greater => {
                i2 += 1;
                if i2 >= len2 {
                    break;
                }
            }
            Equal => {
                out.push(num1);
                i1 += 1;
                i2 += 1;
                if i1 >= len1 || i2 >= len2 {
                    break;
                }
            }
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
