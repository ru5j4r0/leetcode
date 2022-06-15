const DOT: u8 = b'.';
const ONE: u8 = b'1';

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = new_vec();
    let mut cols = new_vec();
    let mut boxes = new_vec();

    for (i, row) in rows.iter_mut().enumerate() {
        for (j, col) in cols.iter_mut().enumerate() {
            let c = board[i][j] as u8;

            if c == DOT {
                continue;
            }

            let bit = 1 << (c - ONE);

            if *row & bit != 0 {
                return false;
            }
            *row |= bit;

            if *col & bit != 0 {
                return false;
            }
            *col |= bit;

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
    vec![0; 9]
}

#[cfg(test)]
mod test {
    use super::*;

    fn test<const M: usize, const N: usize>(board: [[char; N]; M], res: bool) {
        assert_eq!(
            is_valid_sudoku(board.iter().map(|row| Vec::from(*row)).collect()),
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
