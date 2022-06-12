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
mod test_merge_sorted_array {
    fn test(nums1: Vec<i32>, m: i32, nums2: Vec<i32>, n: i32, res: Vec<i32>) {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        super::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, res);
    }

    #[test]
    fn case1() {
        test(
            vec![1, 2, 3, 0, 0, 0],
            3,
            vec![2, 5, 6],
            3,
            vec![1, 2, 2, 3, 5, 6],
        );
    }

    #[test]
    fn case2() {
        test(vec![1], 1, vec![], 0, vec![1]);
    }

    #[test]
    fn case3() {
        test(vec![0], 0, vec![1], 1, vec![1]);
    }
}
