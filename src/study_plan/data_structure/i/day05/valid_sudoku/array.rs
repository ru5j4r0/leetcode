const ONE: usize = '1' as usize;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = new_vec();
    let mut cols = new_vec();
    let mut boxes = new_vec();

    for (i, row) in rows.iter_mut().enumerate() {
        for (j, col) in cols.iter_mut().enumerate() {
            let c = board[i][j];

            if c == '.' {
                continue;
            }

            let n = c as usize - ONE;

            if row[n] {
                return false;
            }
            row[n] = true;

            if col[n] {
                return false;
            }
            col[n] = true;

            let k = j / 3 + i / 3 * 3;
            if boxes[k][n] {
                return false;
            }
            boxes[k][n] = true;
        }
    }

    true
}

fn new_vec() -> Vec<Vec<bool>> {
    let mut vec = Vec::with_capacity(9);
    vec.push(Vec::with_capacity(9));
    vec[0].resize(9, false);
    vec.resize(9, vec[0].clone());
    vec
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
