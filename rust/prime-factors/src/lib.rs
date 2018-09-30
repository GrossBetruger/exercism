extern crate foreach;
extern crate rayon;

use rayon::prelude::*;
use foreach::*;

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    for &prime in primes_to(n).iter() {

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


fn get_hardcoded_primes() -> Vec<u64>{
    vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113, 461, 9539, 894119]
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
    let mut primes = vec![];
    for i in 2..=n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes
}

fn riter() {
}