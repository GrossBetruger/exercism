
pub fn series(digits: &str, len: usize) -> Vec<String> {

    let mut res = vec![];
    let mut offset: usize = 0;

    while (offset as i32 + len as i32 - digits.len() as i32 ) <= 0 {
        let chunk = digits.chars().skip(offset).take(len);
        let s: String = chunk.into_iter().collect::<>();
        offset += 1;
        res.push(s);
    }

    return res;

}
