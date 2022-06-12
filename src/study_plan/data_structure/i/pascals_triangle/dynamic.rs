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

#[cfg(test)]
mod test_pascals_triangle {
    fn test<const N: usize>(num_rows: i32, res: [Vec<i32>; N]) {
        assert_eq!(super::generate(num_rows), Vec::from(res));
    }

    #[test]
    fn case1() {
        test(
            5,
            [
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ],
        );
    }

    #[test]
    fn case2() {
        test(1, [vec![1]]);
    }
}
