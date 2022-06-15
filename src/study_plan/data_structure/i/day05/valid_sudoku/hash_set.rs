use std::collections::HashSet;

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

fn new_vec_set() -> Vec<HashSet<char>> {
    let mut vec_set = Vec::with_capacity(9);
    vec_set.resize(9, HashSet::<char>::new());
    vec_set
}

#[cfg(test)]
mod test {
    fn test<const M: usize, const N: usize>(board: [[char; N]; M], res: bool) {
        assert_eq!(
            super::is_valid_sudoku(board.iter().map(|row| Vec::from(*row)).collect()),
            res
        );
    }

    #[test]
    fn case1() {
        test(
            [
                ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            true,
        );
    }

    #[test]
    fn case2() {
        test(
            [
                ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            false,
        );
    }
}
