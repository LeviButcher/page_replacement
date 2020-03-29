extern crate page_replacement;

use page_replacement::{algorithms, page_report};

fn main() {
    println!("Hello, world!");
    let res = page_report(algorithms::fifo, vec![0, 1, 2, 0, 3], 3);

    println!("{:?}", res);
}
