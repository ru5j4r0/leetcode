use std::cmp::Ordering::*;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n_1 = matrix[0].len() - 1;
    match search_row(&matrix, target, n_1) {
        Some(i) => search_col(&matrix[i], target, n_1),
        None => false,
    }
}

fn search_row(matrix: &[Vec<i32>], target: i32, n_1: usize) -> Option<usize> {
    let m_1 = matrix.len() - 1;
    let mut low = 0;
    let mut high = m_1;

    for _ in 0..=m_1 {
        let i = (high + low) / 2;

        if target < matrix[i][0] {
            if i == 0 {
                return None;
            }
            high = i - 1;
        } else if matrix[i][n_1] < target {
            if m_1 <= i {
                return None;
            }
            low = i + 1;
        } else {
            return Some(i);
        }

        if high <= low {
            return if matrix[low][0] <= target && target <= matrix[low][n_1] {
                Some(low)
            } else {
                None
            };
        }
    }

    None
}

fn search_col(row: &[i32], target: i32, n_1: usize) -> bool {
    let mut low = 0;
    let mut high = n_1;

    for _ in 0..=n_1 {
        let i = (high + low) / 2;
        let value = row[i];

        match value.cmp(&target) {
            Greater => {
                if i == 0 {
                    return false;
                }
                high = i - 1;
            }
            Less => {
                if n_1 <= i {
                    return false;
                }
                low = i + 1;
            }
            Equal => {
                return true;
            }
        }

        if high <= low {
            return row[low] == target;
        }
    }

    false
}

#[cfg(test)]
mod test {
    fn test<const N: usize>(matrix: [Vec<i32>; N], target: i32, res: bool) {
        assert_eq!(super::search_matrix(Vec::from(matrix), target), res);
    }

    #[test]
    fn case1() {
        test(
            [vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3,
            true,
        );
    }

    #[test]
    fn case2() {
        test(
            [vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13,
            false,
        );
    }
}
