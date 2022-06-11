const A: usize = 'a' as usize;

// impl Solution {
pub fn first_uniq_char(s: String) -> i32 {
    let mut arr: [u32; 26] = Default::default();
    let bytes = s.as_bytes();

    for c in s.chars() {
        arr[c as usize - A] += 1;
    }

    for i in 0..s.len() {
        let j = bytes[i] as usize - A;
        if arr[j] == 1 {
            return i as i32;
        }
    }

    -1
}
// }
