pub fn add_if_not_found(mut acc: Vec<u32>, curr: u32) -> Vec<u32> {
    if acc.contains(&curr) {
        return acc;
    }
    acc.push(curr);
    return acc;
}

pub fn remove_first<T>(mut acc: Vec<T>) -> Vec<T> {
    acc.remove(0);
    acc
}

pub fn push<T>(mut acc: Vec<T>, a: T) -> Vec<T> {
    acc.push(a);
    acc
}

pub fn remove<T>(mut acc: Vec<T>, index: usize) -> (T, Vec<T>) {
    let t = acc.remove(index);
    (t, acc)
}
