// impl Solution {
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;
    let mut out = Vec::with_capacity(num_rows);
    out.push(vec![1]);

    for i in 1..num_rows {
        let prev_row = &out[i - 1];
        let mut row = Vec::with_capacity(i + 1);

        row.push(1);
        for j in 1..i {
            row.push(prev_row[j - 1] + prev_row[j]);
        }
        row.push(1);

        out.push(row);
    }

    out
}
// }
