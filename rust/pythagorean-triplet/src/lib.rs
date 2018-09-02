
pub fn find() -> Option<u32> {
    let limit = 1000u32;
    for a in 1..limit {
        for b in 1..limit {
            let c = 1000 - (a + b);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c)
            }
        }
    }
    return None;
}
