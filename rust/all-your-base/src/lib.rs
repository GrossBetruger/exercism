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

    unimplemented!(
        "Convert {:?} from base {} to base {} ten {}",
        number,
        from_base,
        to_base,
        to_num(number, from_base)

    )
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
        // result.push(0);
        for i in 0..=e+1 {
            let digit = base.pow(e) * i;
            println!("i: {}", i);
            println!("e: {}", e);
            println!("digit: {}", digit);
            println!("acc: {}", acc);

            match digit != 0 && acc + digit <= num {
                true => {
                    println!("acc + digit {}", acc + digit);
                    println!("write to: {}", (digits - e) as usize);
                    println!();

                    result[(digits -e ) as usize] += 1;
                    acc += digit;
                }
                false => {println!();}
                }
        }
    }
    println!("res: {:?}", result);
    result

}

