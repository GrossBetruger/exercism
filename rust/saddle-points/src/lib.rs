pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut results = vec![];

    // check if empty
    match input.iter().map(|v| v.len()).max().unwrap() {
        0 => return results,
        _ => {}
    }

    for (i, vec) in input.iter().enumerate() {
        let mut max_indices = vec![];
        let vec_max = vec.iter().max().unwrap();
        for (j, n) in vec.iter().enumerate() {
            if n == vec_max {
                max_indices.push(j)
            }
        }

        for vec_max_idx in max_indices {
            let column_min = input
                .iter()
                .map(|v| v.get(vec_max_idx).unwrap())
                .min()
                .unwrap();

            if column_min == vec_max {
                results.push((i, vec_max_idx));
            }
        }
    }
    return results;
}
