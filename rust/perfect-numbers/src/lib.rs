#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let aliquot_sum = aliquot_sum(num);
    match aliquot_sum > num {
        true => Some(Classification::Abundant),
        false => match aliquot_sum == num {
            true => Some(Classification::Perfect),
            _ => Some(Classification::Deficient),
        },
    }
}

fn aliquot_sum(num: u64) -> u64 {
    (1..num)
        .filter(|x| num % x == 0) // filter factors of num
        .sum() // sum of factors
}
