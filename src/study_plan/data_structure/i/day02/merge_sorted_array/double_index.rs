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

    for i in (0..m + n).rev() {
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

#[cfg(test)]
mod test {
    use super::*;

    fn test<const L: usize, const M: usize, const N: usize>(
        nums1: [i32; L],
        m: i32,
        nums2: [i32; M],
        n: i32,
        res: [i32; N],
    ) {
        let mut nums1 = Vec::from(nums1);
        let mut nums2 = Vec::from(nums2);
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, Vec::from(res));
    }

    #[test]
    fn case1() {
        test([1, 2, 3, 0, 0, 0], 3, [2, 5, 6], 3, [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn case2() {
        test([1], 1, [], 0, [1]);
    }

    #[test]
    fn case3() {
        test([0], 0, [1], 1, [1]);
    }
}
