extern crate page_replacement;

use page_replacement::{algorithms, load_page, utils, MemoryPage, PageReport};
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut page_hit_order = Vec::new();
    for x in 0..100 {
        page_hit_order.insert(x, rng.gen_range(0, 20));
    }
    let algorithms: Vec<(
        &str,
        fn(Vec<MemoryPage>, MemoryPage, Vec<u32>) -> Vec<MemoryPage>,
    )> = vec![
        ("Fifo", algorithms::fifo),
        ("Second Chance", algorithms::second_chance),
        ("Least Recently Use", algorithms::lru),
        ("Not Recently Use", algorithms::nru),
        ("Clock", algorithms::clock),
    ];
    let buffer_sizes = vec![3, 5, 10];

    let algorithms_result = algorithms
        .into_iter()
        .flat_map(|(name, algorithm)| {
            buffer_sizes
                .iter()
                .map(|frame_size| {
                    let (_, page_report, _) = page_hit_order.iter().fold(
                        (vec![], PageReport::new(), vec![]),
                        |(frame, report, past), x| {
                            let (frame, report) =
                                load_page(algorithm, frame, *frame_size, *x, report, past.clone());
                            (frame, report, utils::push(past, *x))
                        },
                    );
                    (name, page_report, *frame_size)
                })
                .collect::<Vec<(&str, PageReport, u32)>>()
        })
        .collect::<Vec<(&str, PageReport, u32)>>();

    println!("Ran With: {:?}", page_hit_order);

    for (name, report, frame_size) in algorithms_result {
        println!("| {} | frame_size: {} {}", name, frame_size, report);
    }
}
