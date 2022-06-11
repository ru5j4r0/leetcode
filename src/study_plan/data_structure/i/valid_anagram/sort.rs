// impl Solution {
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
// }
