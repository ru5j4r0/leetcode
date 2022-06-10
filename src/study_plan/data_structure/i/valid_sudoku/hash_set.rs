use std::collections::HashSet;

// impl Solution {
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row_set = new_vec_set();
    let mut col_set = new_vec_set();
    let mut box_set = new_vec_set();

    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];

            if c == '.' {
                continue;
            }

            if !row_set[i].insert(c) {
                return false;
            }

            if !col_set[j].insert(c) {
                return false;
            }

            if !box_set[j / 3 + i / 3 * 3].insert(c) {
                return false;
            }
        }
    }

    true
}
// }

fn new_vec_set() -> Vec<HashSet<char>> {
    let mut vec_set = Vec::with_capacity(9);
    vec_set.resize(9, HashSet::<char>::new());
    vec_set
}
