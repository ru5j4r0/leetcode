const A: usize = 'a' as usize;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut arr: [u32; 26] = Default::default();

    for c in s.chars() {
        arr[c as usize - A] += 1;
    }

    for c in t.chars() {
        let value = &mut arr[c as usize - A];
        if *value == 0 {
            return false;
        }
        *value -= 1;
    }

    true
}

#[cfg(test)]
mod test_valid_anagram {
    fn test(s: &str, t: &str, res: bool) {
        assert_eq!(super::is_anagram(s.to_string(), t.to_string()), res);
    }

    #[test]
    fn case1() {
        test("anagram", "nagaram", true);
    }

    #[test]
    fn case2() {
        test("rat", "car", false);
    }
}
