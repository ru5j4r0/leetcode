const A: usize = 'a' as usize;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut arr: [u32; 26] = Default::default();

    for c in magazine.chars() {
        arr[c as usize - A] += 1;
    }

    for c in ransom_note.chars() {
        let value = &mut arr[c as usize - A];
        if *value == 0 {
            return false;
        }
        *value -= 1;
    }

    true
}

#[cfg(test)]
mod test {
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
