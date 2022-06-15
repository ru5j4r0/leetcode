const A: usize = 'a' as usize;

pub fn first_uniq_char(s: String) -> i32 {
    let mut arr: [u32; 26] = Default::default();
    let bytes = s.as_bytes();

    for c in s.chars() {
        arr[c as usize - A] += 1;
    }

    for (i, byte) in bytes.iter().enumerate() {
        let j = *byte as usize - A;
        if arr[j] == 1 {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    fn test(s: &str, res: i32) {
        assert_eq!(super::first_uniq_char(s.to_string()), res);
    }

    #[test]
    fn case1() {
        test("leetcode", 0);
    }

    #[test]
    fn case2() {
        test("loveleetcode", 2);
    }

    #[test]
    fn case3() {
        test("aabb", -1);
    }
}
