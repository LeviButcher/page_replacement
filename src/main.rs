extern crate page_replacement;

use page_replacement::{algorithms, load_page, utils, MemoryPage, PageReport};

fn main() {
    let algorithms: Vec<(
        &str,
        fn(Vec<MemoryPage>, MemoryPage, Vec<u32>) -> Vec<MemoryPage>,
    )> = vec![
        ("Fifo", algorithms::fifo),
        ("Second Chance", algorithms::second_chance),
        ("Least Recently Use", algorithms::lru),
    ];
    let buffer_sizes = vec![3, 5, 10];
    let page_hit_order = vec![1, 2, 4, 2, 1, 5, 4];

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
                    (name, page_report)
                })
                .collect::<Vec<(&str, PageReport)>>()
        })
        .collect::<Vec<(&str, PageReport)>>();

    println!("Ran With: {:?}", page_hit_order);

    for (name, report) in algorithms_result {
        println!("| {} {}", name, report);
    }
}
