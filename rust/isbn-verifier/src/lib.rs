/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if invalid_isbn(isbn) {
        return false;
    }

    let checksum = calc_checksum(isbn);
    checksum % 11 == 0
}


fn calc_checksum(isbn: &str) -> i32 {
    let digits = isbn.chars().filter(|c| c.is_digit(10) || *c == 'X').collect::<Vec<char>>();
    let length = digits.len();
    let checksum = digits.iter().enumerate().fold(0, |acc, (i, c)| {
        let digit = match c {
            'X' => 10,
            _ => c.to_string().parse::<i32>().unwrap()
        };

        let position = length - i;
        acc + (position as i32 * digit)
    });
    checksum
}

fn invalid_isbn(isbn: &str) -> bool {
    let length = isbn.len();
    isbn.chars().enumerate().any(|(i, c)|
        ! (c.is_digit(10)
            || c == '-'
            || i == (length)-1 && c == 'X'))
}