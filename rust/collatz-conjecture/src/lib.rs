pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => Some(steps_to_1(n))
    }
}

fn steps_to_1(mut n: u64) -> u64 {
    let mut counter = 0;

    while n != 1 {
        match n % 2 {
            0 => {
                n = n / 2;
                counter += 1
            },
            _ => {
                n = n * 3 + 1;
                counter += 1
            }
        }
    }
    counter
}

