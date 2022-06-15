use std::collections::VecDeque;

pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let r = r as usize;
    let c = c as usize;
    let size = r * c;
    if mat.len() * mat[0].len() != size {
        return mat;
    }

    let mut queue = VecDeque::with_capacity(size);

    for row in mat {
        for num in row {
            queue.push_back(num);
        }
    }

    let mut out = Vec::with_capacity(r);
    out.push(Vec::with_capacity(c));
    out[0].resize(c, Default::default());
    out.resize(r, out[0].clone());

    for row in out.iter_mut() {
        for num in row.iter_mut() {
            *num = queue.pop_front().unwrap();
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    fn test<const M: usize, const N: usize>(
        mat: [Vec<i32>; M],
        r: i32,
        c: i32,
        res: [Vec<i32>; N],
    ) {
        assert_eq!(matrix_reshape(Vec::from(mat), r, c), Vec::from(res));
    }

    #[test]
    fn case1() {
        test([vec![1, 2], vec![3, 4]], 1, 4, [vec![1, 2, 3, 4]]);
    }

    #[test]
    fn case2() {
        test([vec![1, 2], vec![3, 4]], 2, 4, [vec![1, 2], vec![3, 4]]);
    }
}
