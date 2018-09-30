extern crate foreach;
extern crate rayon;

use foreach::*;
use rayon::prelude::*;

const LIMIT: u64 = 1000_000;

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    for &prime in primes_to(LIMIT).iter() {
        while n % prime == 0 {
            factors.push(prime);
            n /= prime;
        }

        if n == 1 {
            break;
        }
    }
    factors
}

fn is_prime(n: u64) -> bool {
    let mut prime = true;
    let mut iter = 2..=((n as f64).sqrt()) as u64;
    iter.foreach(|i, _| {
        if n % i == 0 {
            prime = false;
            return;
        }
    });
    prime
}

pub fn primes_to(n: u64) -> Vec<u64> {
    (2..n + 1)
        .into_par_iter()
        .filter(|i| is_prime(*i))
        .collect()
}
