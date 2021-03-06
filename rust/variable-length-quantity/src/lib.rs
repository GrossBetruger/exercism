extern crate itertools;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = vec![];

    for val in values.iter() {
        let bin_representation = to_bin(*val);
        match bin_representation.len() < 8 {
            true => res.push(bin_to_byte(&bin_representation.chars().collect())),
            false => {
                let mut sevens = vec![];
                for seven in bin_representation.chars().rev().chunks(7).into_iter() {
                    sevens.push(seven.collect::<String>());
                }

                for (lsb, msbs) in sevens.split_first() {
                    for msb in msbs.iter().rev() {
                        let msb = msb.chars().rev().collect::<String>();
                        let msb_str: Vec<char> = pad_byte(&msb, "1").chars().collect();
                        res.push(bin_to_byte(&msb_str));

                    }
                    let lsb = lsb.chars().rev().collect::<String>();
                    res.push(bin_to_byte(&pad_byte(&lsb, "0").chars().collect()));

                }
            }
        }
    }

    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes.iter().all(|b| *b & 128 == 128) {
        return Err(Error::IncompleteNumber);
    }
    if bytes.len() == 1 {
        return Ok(vec![*bytes.get(0).unwrap() as u32]);
    }

    let mut results = vec![];
    let groups = bytes.iter()
        .group_by(|b| *b & 128 == 128)
        .into_iter().map(|(_pred, group)| group.cloned().collect()).collect::<Vec<Vec<u8>>>();

    let nums_iter = groups.iter().chunks(2);

    for num in nums_iter.into_iter() {
        let num_parts = num.collect_vec();
        let msbs: Vec<u8> = num_parts.get(0).unwrap().to_vec();
        let number_of_lsbs = num_parts.get(1).unwrap().len();

        match number_of_lsbs {
            1 => {
                let lsb: u8 = *num_parts.get(1).unwrap().get(0).unwrap();
                let calculated_num = parse_single_number(&msbs, lsb);
                if calculated_num  > u32::max_value() as u64 {
                    return Err(Error::Overflow);
                }

                results.push(calculated_num as u32);
            }
            _ => {
                let lsb: u8 = *num_parts.get(1).unwrap().get(0).unwrap();
                let calculated_num = parse_single_number(&msbs, lsb);
                if calculated_num  > u32::max_value() as u64 {
                    return Err(Error::Overflow);
                }

                results.push(calculated_num as u32);

                for lsb in num_parts.get(1).unwrap().iter().skip(1) {
                    results.push(*lsb as u32)
                }

            }
        }

    }

    Ok(results)
}

fn parse_single_number(bytes: &[u8], lsb: u8) -> u64 {
    let mut stream = String::new();
    for byte in bytes.iter() {
        stream.push_str(&skip_msb(&byte_to_bin(*byte)))
    }
    let least_significant_byte = &skip_msb(&byte_to_bin(lsb));
    stream.push_str(least_significant_byte);
    bin_to_num64(&stream.chars().collect()) as u64
}

fn byte_to_bin(num: u8) -> String {
    let raw_repr = format!("{:b}", num);
    match raw_repr.len() == 8 {
        true => raw_repr,
        _ => pad_byte(&format!("{:b}", num),"0")
    }

}

fn to_bin(num: u32) -> String {
    format!("{:b}", num)
}

fn skip_msb(byte_repr: &str) -> String {
    byte_repr.chars().skip(1).collect()
}

fn pad_byte(byte_repr: &str, first: &str) -> String {
    let padding = String::from("0").repeat(7 - byte_repr.len());
    format!("{}{}{}", first, padding, byte_repr)

}

fn bin_to_byte(bin: &Vec<char>) -> u8 {
    bin.iter().rev().enumerate().fold(0_u8, {
        |acc: u8, (e, bin_dig)| acc + bin_dig.to_digit(2).unwrap() as u8 * 2_u8.pow(e as u32)
    })
}

fn bin_to_num64(bin: &Vec<char>) -> u64 {
    bin.iter().rev().enumerate().fold(0_u64, {
        |acc: u64, (e, bin_dig)| acc + bin_dig.to_digit(2).unwrap() as u64 * 2_u64.pow(e as u32)
    })
}
