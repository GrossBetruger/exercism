pub fn square_of_sum(n: usize) -> usize {
    (1..=n).fold(0, |a, b| a + b).pow(2)
}

pub fn sum_of_squares(n: usize) -> usize {
    (1..=n as i64)
        .collect::<Vec<i64>>()
        .iter()
        .map(|a| a.pow(2))
        .sum::<i64>() as usize
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
