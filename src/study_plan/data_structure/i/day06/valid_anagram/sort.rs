pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut str1 = Vec::from(s);
    str1.sort_unstable();

    let mut str2 = Vec::from(t);
    str2.sort_unstable();

    str1 == str2
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(s: &str, t: &str, res: bool) {
        assert_eq!(is_anagram(s.to_string(), t.to_string()), res);
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
