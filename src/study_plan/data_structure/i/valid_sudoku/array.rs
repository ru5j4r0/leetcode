const ONE: usize = '1' as usize;

// impl Solution {
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = new_vec();
    let mut cols = new_vec();
    let mut boxes = new_vec();

    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];

            if c == '.' {
                continue;
            }

            let n = c as usize - ONE;

            if rows[i][n] {
                return false;
            }
            rows[i][n] = true;

            if cols[j][n] {
                return false;
            }
            cols[j][n] = true;

            let k = j / 3 + i / 3 * 3;
            if boxes[k][n] {
                return false;
            }
            boxes[k][n] = true;
        }
    }

    true
}
// }

fn new_vec() -> Vec<Vec<bool>> {
    let mut vec = Vec::with_capacity(9);
    vec.push(Vec::with_capacity(9));
    vec[0].resize(9, false);
    vec.resize(9, vec[0].clone());
    vec
}
