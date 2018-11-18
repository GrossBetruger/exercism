pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut results = vec![];

    // check if empty
    match input.iter().all(|v| v.is_empty()) {
        true => return results,
        false => {}
    }

    for (i, vec) in input.iter().enumerate() {
        let vec_max = vec.iter().max().unwrap();
        let max_indices = vec.iter().enumerate().filter(|(_j, n)| *n == vec_max).map(|(j, _n)| j);

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
