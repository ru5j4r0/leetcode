use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map = HashMap::new();

    for c in magazine.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for c in ransom_note.chars() {
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
mod test_ransom_note {
    fn test(ransom_note: &str, magazine: &str, res: bool) {
        assert_eq!(
            super::can_construct(ransom_note.to_string(), magazine.to_string()),
            res
        );
    }

    #[test]
    fn case1() {
        test("a", "b", false);
    }

    #[test]
    fn case2() {
        test("aa", "ab", false);
    }

    #[test]
    fn case3() {
        test("aa", "aab", true);
    }
}
