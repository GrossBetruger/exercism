extern crate rand;

use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}

fn modular_exponentiation(b: u64, e: u64, m: u64) -> u64 {
    let mut e_tag = 0;
    let mut solution = 1;

    while e_tag < e {
        e_tag += 1;
        solution = (b * solution) % m;
    }
    solution
}
