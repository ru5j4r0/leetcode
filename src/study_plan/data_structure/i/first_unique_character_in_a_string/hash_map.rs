use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let mut map = HashMap::new();
    let bytes = s.as_bytes();

    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for i in 0..s.len() {
        if *map.get(&(bytes[i] as char)).unwrap() == 1 {
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
