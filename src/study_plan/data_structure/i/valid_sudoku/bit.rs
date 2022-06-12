const DOT: u8 = '.' as u8;
const ONE: u8 = '1' as u8;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = new_vec();
    let mut cols = new_vec();
    let mut boxes = new_vec();

    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j] as u8;

            if c == DOT {
                continue;
            }

            let bit = 1 << (c - ONE);

            if rows[i] & bit != 0 {
                return false;
            }
            rows[i] |= bit;

            if cols[j] & bit != 0 {
                return false;
            }
            cols[j] |= bit;

            let k = j / 3 + i / 3 * 3;
            if boxes[k] & bit != 0 {
                return false;
            }
            boxes[k] |= bit;
        }
    }

    true
}

fn new_vec() -> Vec<u32> {
    let mut vec = Vec::with_capacity(9);
    vec.resize(9, 0 as u32);
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
