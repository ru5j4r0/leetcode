use std::cmp::Ordering::*;

// impl Solution {
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix[0].len();
    let size_1 = matrix.len() * n - 1;
    let mut low = 0;
    let mut high = size_1;

    for _ in 0..=size_1 {
        let i = (high + low) / 2;

        match matrix[i / n][i % n].cmp(&target) {
            Equal => {
                return true;
            }
            Less => {
                if i >= size_1 {
                    return false;
                }
                low = i + 1;
            }
            Greater => {
                if i <= 0 {
                    return false;
                }
                high = i - 1;
            }
        }

        if low >= high {
            return matrix[low / n][low % n] == target;
        }
    }

    false
}
// }
