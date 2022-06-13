use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map = HashMap::new();

    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        match map.get_mut(&c) {
            Some(value) => {
                if *value == 0 {
                    return false;
                }
                *value -= 1;
            }
            None => return false,
        }
    }

    true
}

#[cfg(test)]
mod test {
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
