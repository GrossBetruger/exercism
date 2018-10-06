const RADIX: u32 = 10;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap())
        .collect();

    let number_of_digits = digits.len();

    let armstrong_sum = digits
        .iter()
        .fold(0, |a, b| a + b.pow(number_of_digits as u32));

    num == armstrong_sum
}
