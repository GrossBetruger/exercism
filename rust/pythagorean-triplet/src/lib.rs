const LIMIT: u32 = 1000;

pub fn find() -> Option<u32> {
    for a in 1..LIMIT {
        for b in 1..LIMIT-a {
            let c = 1000 - (a + b);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c)
            }
        }
    }
    return None;
}
