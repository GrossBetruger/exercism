extern crate foreach;

use foreach::*;

const LIMIT: u32 = 1_000_000;

pub fn nth(n: u32) -> Option<u32> {
    let mut prime_count = 0;
    for i in 3..LIMIT {
        if is_prime(i) {
            prime_count += 1;
            if prime_count + 1 == n {
                return Some(i)
            }
        }
    }
    return None
}

fn is_prime(n: u32) -> bool {
    let mut prime = true;
    let mut iter = 2..((n as f32).sqrt()) as u32 +1;
    iter.foreach(|i, _| {
        if n % i == 0 {
            prime = false;
            return;
        }
    });
    prime
}