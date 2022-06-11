use std::collections::HashMap;

// impl Solution {
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
// }
