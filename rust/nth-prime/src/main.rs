extern crate nth_prime;

use nth_prime::*;

fn main() {

    let ord = 10000;
    println!("prime number: {}, {}", ord, nth(ord).unwrap());
}