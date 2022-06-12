const ONE: usize = '1' as usize;

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

fn new_vec() -> Vec<Vec<bool>> {
    let mut vec = Vec::with_capacity(9);
    vec.push(Vec::with_capacity(9));
    vec[0].resize(9, false);
    vec.resize(9, vec[0].clone());
    vec
}

#[cfg(test)]
mod test_valid_sudoku {
    fn test(board: Vec<Vec<char>>, res: bool) {
        assert_eq!(super::is_valid_sudoku(board), res);
    }

    #[test]
    fn case1() {
        test(
            vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            true,
        );
    }

    #[test]
    fn case2() {
        test(
            vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            false,
        );
    }
}
