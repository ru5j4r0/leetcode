use std::collections::HashMap;

// impl Solution {
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
// }
