

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if vec![1,0].contains(&from_base) {
        return Err(Error::InvalidInputBase);
    }

    if vec![1,0].contains(&to_base) {
        return Err(Error::InvalidOutputBase);
    }

    if number.iter().any(|n| n >= &from_base) {
        return Err(Error::InvalidDigit(*number.iter().filter(|n| n >= &&from_base).nth(0).unwrap()));
    }

    let num = to_num(number, from_base);
    let target = from_num(num, to_base);
    Ok(target)
}

fn to_num(num: &[u32], base: u32) -> u32 {
    num.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, n)|acc + (base.pow(i as u32) * n))
}

fn from_num(num: u32, base: u32) -> Vec<u32> {
    let digits = (num as f32).log(base as f32).ceil() as u32;
    let mut result: Vec<u32> = Vec::with_capacity(digits as usize);
    let mut acc = 0;

    for _ in 0..=digits {
        result.push(0
        );
    }
    for e in (0..=digits).rev() {
        for _ in 0..base {

            let digit = base.pow(e);

            match digit != 0 && acc + digit <= num {
                true => {


                    result[(digits -e ) as usize] += 1;
                    acc += digit;
                }
                false => {}
                }
        }
    }
    result.iter().skip_while(|x| **x == 0).map(|x| *x).collect::<Vec<u32>>()

}

