pub fn square_of_sum(n: usize) -> usize {
    (1_i64..n as i64 + 1).fold(0, |a, b| a + b).pow(2) as usize
}

pub fn sum_of_squares(n: usize) -> usize {
    let sum_of_squares: i64 = (1..n as i64 + 1)
        .collect::<Vec<i64>>()
        .iter()
        .map(|a| a.pow(2))
        .sum();
    sum_of_squares as usize
}

pub fn difference(n: usize) -> usize {
    (square_of_sum(n) - sum_of_squares(n))
}
