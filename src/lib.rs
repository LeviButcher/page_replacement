pub mod algorithms;
mod utils;

use utils::push;

#[derive(Copy, Clone, Debug)]
pub struct MemoryPage {
    number: u32,
    present: bool,
    referenced: bool,
}

impl MemoryPage {
    fn new(number: u32) -> MemoryPage {
        MemoryPage {
            number,
            present: false,
            referenced: false,
        }
    }

    fn referenced(self) -> MemoryPage {
        MemoryPage {
            number: self.number,
            present: self.present,
            referenced: true,
        }
    }

    fn clear(self) -> MemoryPage {
        MemoryPage {
            number: self.number,
            present: self.present,
            referenced: false,
        }
    }
}

impl PartialEq for MemoryPage {
    fn eq(&self, rhs: &MemoryPage) -> bool {
        self.number == rhs.number
    }
}

#[derive(PartialEq, Debug)]
pub struct PageReport {
    hits: u32,
    faults: u32,
    removed: u32,
}

impl PageReport {
    fn new() -> PageReport {
        PageReport {
            faults: 0,
            hits: 0,
            removed: 0,
        }
    }
    fn fault(self) -> PageReport {
        PageReport {
            hits: self.hits,
            faults: self.faults + 1,
            removed: self.removed,
        }
    }
    fn hit(self) -> PageReport {
        PageReport {
            hits: self.hits + 1,
            faults: self.faults,
            removed: self.removed,
        }
    }
    fn removed(self) -> PageReport {
        PageReport {
            hits: self.hits,
            faults: self.faults,
            removed: self.removed + 1,
        }
    }
}

pub fn page_report<F>(handle_loading: F, page_hit_order: Vec<u32>, frame_size: u32) -> PageReport
where
    F: Fn(Vec<MemoryPage>, MemoryPage, Vec<u32>) -> Vec<MemoryPage>,
{
    let page_report = PageReport::new();
    let page_frames = Vec::<MemoryPage>::new();
    let past_pages = Vec::<u32>::new();

    page_report_recursion(
        page_hit_order,
        page_frames,
        page_report,
        frame_size,
        handle_loading,
        past_pages,
    )
}

fn page_report_recursion<F>(
    page_hit_order: Vec<u32>,
    page_frames: Vec<MemoryPage>,
    page_report: PageReport,
    frame_size: u32,
    handle_loading: F,
    past_pages: Vec<u32>,
) -> PageReport
where
    F: Fn(Vec<MemoryPage>, MemoryPage, Vec<u32>) -> Vec<MemoryPage>,
{
    let page_number = page_hit_order.as_slice().split_first();

    match page_number {
        None => return page_report,
        Some((page_number, page_hit_order)) => {
            let page_hit_order = page_hit_order.to_vec();
            let page = MemoryPage::new(*page_number);

            // Page is in Memory
            if page_frames.contains(&page) {
                let page_frames = page_frames
                    .iter()
                    .map(|x| {
                        if x.number == *page_number {
                            return x.referenced();
                        }
                        *x
                    })
                    .collect();

                return page_report_recursion(
                    page_hit_order,
                    page_frames,
                    page_report.hit(),
                    frame_size,
                    handle_loading,
                    push(past_pages, *page_number),
                );
            }

            // Room to load page in memory
            if (page_frames.len() as u32) < frame_size {
                return page_report_recursion(
                    page_hit_order,
                    push(page_frames, page),
                    page_report.fault(),
                    frame_size,
                    handle_loading,
                    push(past_pages, *page_number),
                );
            }

            // No Room, Call algorithm to add to page_frames
            let page_frames = handle_loading(page_frames, page, past_pages.clone());

            return page_report_recursion(
                page_hit_order,
                page_frames,
                page_report.fault().removed(),
                frame_size,
                handle_loading,
                push(past_pages, *page_number),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{algorithms, *};

    #[test]
    fn page_report_fifo_should_return_expected() {
        let page_hit_order = vec![0, 1, 2, 3];
        let expected_page_report = PageReport {
            hits: 0,
            faults: 4,
            removed: 1,
        };
        let frame_size = 3;
        let res = page_report(algorithms::fifo, page_hit_order, frame_size);
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_fifo_repeat_list_return_expected() {
        let page_hit_order = vec![0, 1, 0, 1, 1, 0, 1];
        let expected_page_report = PageReport {
            hits: 5,
            faults: 2,
            removed: 0,
        };
        let frame_size = 2;
        let res = page_report(algorithms::fifo, page_hit_order, frame_size);
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_fifo_complex_case_should_return_expected() {
        let page_hit_order = vec![1, 3, 0, 3, 5, 6, 3];
        let expected_page_report = PageReport {
            hits: 1,
            faults: 6,
            removed: 3,
        };
        let frame_size = 3;
        let res = page_report(algorithms::fifo, page_hit_order, frame_size);
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_lru_should_return_expected() {
        let page_hit_order = vec![0, 1, 2, 0, 3, 2, 1];
        let expected_page_report = PageReport {
            hits: 2,
            faults: 5,
            removed: 2,
        };
        let frame_size = 3;
        let res = page_report(algorithms::lru, page_hit_order, frame_size);
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_lru_complex_case_should_return_expected() {
        let page_hit_order = vec![7, 0, 1, 2, 0, 3, 0, 4, 2, 3, 0, 3, 2, 3];
        let expected_page_report = PageReport {
            hits: 8,
            faults: 6,
            removed: 2,
        };
        let frame_size = 4;
        let res = page_report(algorithms::lru, page_hit_order, frame_size);
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_second_chance_should_return_expected() {
        let page_hit_order = vec![0, 4, 1, 4, 2, 4, 3, 4, 2, 4, 0];
        let expected_page_report = PageReport {
            hits: 5,
            faults: 6,
            removed: 3,
        };
        let frame_size = 3;
        let res = page_report(algorithms::second_chance, page_hit_order, frame_size);
        assert_eq!(res, expected_page_report);
    }
}
