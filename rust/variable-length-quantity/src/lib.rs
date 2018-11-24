#[macro_use] extern crate itertools;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut sevens = vec![];
    for val in values.iter() {
        for c in to_bin(*val).chars().chunks(7).into_iter() {
//            println!("chunk: {:?}", c.collect_vec());
            sevens.push(c.collect_vec());
        }
    }

    vec![]
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}

fn to_bin(num: u32) -> String {
    format!("{:b}", num)
}

fn bin_to_num(bin: &str) -> u32 {
    bin.chars().rev().enumerate().fold(0, {
        |acc, (e, bin_dig)| acc + bin_dig.to_digit(2).unwrap() * 2_u32.pow(e as u32)
    })
}
