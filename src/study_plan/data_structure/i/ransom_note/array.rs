const A: usize = 'a' as usize;

// impl Solution {
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
// }
