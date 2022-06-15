use std::cmp::Ordering::*;

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
                if i == 0 {
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
