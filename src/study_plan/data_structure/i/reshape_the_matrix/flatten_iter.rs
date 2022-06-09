// impl Solution {
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
// }
