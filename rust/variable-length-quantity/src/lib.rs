#[macro_use] extern crate itertools;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    for val in values.iter() {
        for c in to_bin(*val).chars().chunks(3).into_iter() {
            println!("chunk: {:?}", c.collect_vec());
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
