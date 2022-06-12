pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let r = r as usize;
    let c = c as usize;
    if mat.len() * mat[0].len() != r * c {
        return mat;
    }

    let mut out = Vec::with_capacity(r);
    out.push(Vec::with_capacity(c));
    out[0].resize(c, Default::default());
    out.resize(r, out[0].clone());

    for (i, o) in mat.iter().flatten().zip(out.iter_mut().flatten()) {
        *o = *i;
    }

    out
}

#[cfg(test)]
mod test_reshape_the_matrix {
    fn test(mat: Vec<Vec<i32>>, r: i32, c: i32, res: Vec<Vec<i32>>) {
        assert_eq!(super::matrix_reshape(mat, r, c), res);
    }

    #[test]
    fn case1() {
        test(vec![vec![1, 2], vec![3, 4]], 1, 4, vec![vec![1, 2, 3, 4]]);
    }

    #[test]
    fn case2() {
        test(
            vec![vec![1, 2], vec![3, 4]],
            2,
            4,
            vec![vec![1, 2], vec![3, 4]],
        );
    }
}
