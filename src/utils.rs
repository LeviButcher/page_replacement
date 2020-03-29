pub fn return_largest(a: u32, b: &u32) -> u32 {
    if a < *b {
        return *b;
    }
    a
}

pub fn add_if_not_found(mut acc: Vec<u32>, curr: u32) -> Vec<u32> {
    if acc.contains(&curr) {
        return acc;
    }
    acc.push(curr);
    return acc;
}
