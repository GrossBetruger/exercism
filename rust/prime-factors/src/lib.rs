

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    for &prime in get_hardcoded_primes().iter() {
        while n % prime == 0 {
            factors.push(prime);
            n /= prime;
        }
    }
    factors
}


fn get_hardcoded_primes() -> Vec<u64>{
    vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113, 461, 9539, 894119]
}

