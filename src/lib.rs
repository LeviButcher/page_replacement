use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
struct MemoryPage {
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
}

#[derive(PartialEq, Debug)]
pub struct PageReport {
    hits: u32,
    faults: u32,
    removed: u32,
}

impl PageReport {
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

fn generate_pages(page_size: u32) -> Vec<MemoryPage> {
    (0..page_size)
        .collect::<Vec<u32>>()
        .iter()
        .map(|i| MemoryPage::new(*i))
        .collect()
}

fn return_largest(a: u32, b: &u32) -> u32 {
    if a < *b {
        return *b;
    }
    a
}

pub fn page_report_fifo(page_hit_order: Vec<u32>, frame_size: u32) -> PageReport {
    // Get The First Element in page_hit_order
    // If page is not present
    //  if frame has room, add to end, and increment faults
    //  else overwrite oldest frame with new frame, and increment faults and removed
    // else Move to next page_hit_order and increment hits

    let page_report = PageReport {
        faults: 0,
        hits: 0,
        removed: 0,
    };
    let page_size = page_hit_order.iter().fold(0, return_largest) + 1;
    let page_frames = VecDeque::<&MemoryPage>::new();
    let page_table = generate_pages(page_size);

    page_report_fifo_recursion(
        page_hit_order,
        page_frames,
        page_report,
        &page_table,
        frame_size,
    )
}

fn page_report_fifo_recursion<'a>(
    page_hit_order: Vec<u32>,
    mut page_frames: VecDeque<&'a MemoryPage>,
    page_report: PageReport,
    page_table: &'a Vec<MemoryPage>,
    frame_size: u32,
) -> PageReport {
    let page_number = page_hit_order.as_slice().split_first();

    match page_number {
        None => return page_report,
        Some((page_number, page_hit_order)) => {
            let page_hit_order = page_hit_order.to_vec();
            let page = page_table
                .iter()
                .find(|x| x.number == *page_number)
                .unwrap();

            if !page_frames.iter().any(|x| x.number == *page_number) {
                if (page_frames.len() as u32) < frame_size {
                    page_frames.push_back(page);
                    return page_report_fifo_recursion(
                        page_hit_order,
                        page_frames,
                        page_report.fault(),
                        page_table,
                        frame_size,
                    );
                } else {
                    page_frames.push_front(page);
                    page_frames.pop_back();

                    return page_report_fifo_recursion(
                        page_hit_order,
                        page_frames,
                        page_report.fault().removed(),
                        page_table,
                        frame_size,
                    );
                }
            } else {
                return page_report_fifo_recursion(
                    page_hit_order,
                    page_frames,
                    page_report.hit(),
                    page_table,
                    frame_size,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_report_fifo_should_return_expected() {
        let page_hit_order = vec![0, 1, 2, 3];
        let expected_page_report = PageReport {
            hits: 0,
            faults: 4,
            removed: 1,
        };
        let frame_size = 3;
        let res = page_report_fifo(page_hit_order, frame_size);
        assert_eq!(res, expected_page_report);
    }
}
