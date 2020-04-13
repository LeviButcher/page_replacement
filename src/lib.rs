pub mod algorithms;
pub mod utils;

use std::fmt;
use utils::push;

#[derive(Copy, Clone, Debug)]
pub struct MemoryPage {
    number: u32,
    present: bool,
    referenced: bool,
    modified: bool,
}

impl MemoryPage {
    fn new(number: u32) -> MemoryPage {
        MemoryPage {
            number,
            present: false,
            referenced: false,
            modified: false,
        }
    }

    fn referenced(self) -> MemoryPage {
        MemoryPage {
            number: self.number,
            present: self.present,
            referenced: true,
            modified: false,
        }
    }

    fn modified(self) -> MemoryPage {
        MemoryPage {
            number: self.number,
            present: self.present,
            referenced: false,
            modified: true,
        }
    }

    fn modified_and_referenced(self) -> MemoryPage {
        MemoryPage {
            number: self.number,
            present: self.present,
            referenced: true,
            modified: true,
        }
    }

    fn clear(self) -> MemoryPage {
        MemoryPage {
            number: self.number,
            present: self.present,
            referenced: false,
            modified: false,
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
    pub fn new() -> PageReport {
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

impl fmt::Display for PageReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "| hits: {} | faults: {} | removed: {} |",
            self.hits, self.faults, self.removed
        )
    }
}

pub fn load_page<F>(
    handle_loading: F,
    page_frames: Vec<MemoryPage>,
    frame_size: u32,
    page_hit: u32,
    report: PageReport,
    past_pages: Vec<u32>,
) -> (Vec<MemoryPage>, PageReport)
where
    F: Fn(Vec<MemoryPage>, MemoryPage, Vec<u32>) -> Vec<MemoryPage>,
{
    let page = MemoryPage::new(page_hit);
    // Page is in Memory
    if page_frames.contains(&page) {
        let page_frames = page_frames
            .iter()
            .map(|x| {
                if x.number == page_hit {
                    return x.referenced();
                }
                *x
            })
            .collect();
        return (page_frames, report.hit());
    }

    // Room to load page in memory
    if (page_frames.len() as u32) < frame_size {
        return (push(page_frames, page), report.fault());
    }

    // No Room, replace a page
    let page_frames = handle_loading(page_frames, page, past_pages);
    (page_frames, report.fault().removed())
}

#[cfg(test)]
mod tests {
    use super::{algorithms, *};

    #[test]
    fn page_report_fifo_should_return_expected() {
        let page_hit = 0;
        let frame_size = 3;
        let frame = vec![];
        let expected_page_report = PageReport {
            hits: 0,
            faults: 1,
            removed: 0,
        };
        let report = PageReport::new();
        let (_, res) = load_page(
            algorithms::fifo,
            frame,
            frame_size,
            page_hit,
            report,
            vec![],
        );
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_fifo_repeat_list_return_expected() {
        let page_hit = 0;
        let frame_size = 2;
        let frame = vec![MemoryPage::new(0), MemoryPage::new(1)];
        let expected_page_report = PageReport {
            hits: 5,
            faults: 2,
            removed: 0,
        };
        let report = PageReport {
            hits: 4,
            faults: 2,
            removed: 0,
        };
        let (_, res) = load_page(
            algorithms::fifo,
            frame,
            frame_size,
            page_hit,
            report,
            vec![],
        );
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_fifo_complex_case_should_return_expected() {
        let page_hit = 3;
        let frame_size = 3;
        let frame = vec![MemoryPage::new(0), MemoryPage::new(5), MemoryPage::new(6)];
        let past_pages = vec![1, 3, 0, 3, 5, 6];
        let report = PageReport {
            hits: 1,
            faults: 5,
            removed: 2,
        };
        let expected_page_report = PageReport {
            hits: 1,
            faults: 6,
            removed: 3,
        };
        let (_, res) = load_page(
            algorithms::fifo,
            frame,
            frame_size,
            page_hit,
            report,
            past_pages,
        );
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_lru_should_return_expected() {
        let page_hit = 1;
        let frame_size = 3;
        let frame = vec![MemoryPage::new(0), MemoryPage::new(2), MemoryPage::new(3)];
        let past_pages = vec![0, 1, 2, 0, 3, 2];
        let report = PageReport {
            hits: 2,
            faults: 4,
            removed: 1,
        };
        let expected_page_report = PageReport {
            hits: 2,
            faults: 5,
            removed: 2,
        };
        let (_, res) = load_page(
            algorithms::lru,
            frame,
            frame_size,
            page_hit,
            report,
            past_pages,
        );
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_lru_complex_case_should_return_expected() {
        let page_hit = 3;
        let frame_size = 4;
        let frame = vec![
            MemoryPage::new(3),
            MemoryPage::new(0),
            MemoryPage::new(4),
            MemoryPage::new(2),
        ];
        let past_pages = vec![7, 0, 1, 2, 0, 3, 0, 4, 2, 3, 0, 3, 2];
        let report = PageReport {
            hits: 7,
            faults: 6,
            removed: 2,
        };
        let expected_page_report = PageReport {
            hits: 8,
            faults: 6,
            removed: 2,
        };
        let (_, res) = load_page(
            algorithms::lru,
            frame,
            frame_size,
            page_hit,
            report,
            past_pages,
        );
        assert_eq!(res, expected_page_report);
    }

    #[test]
    fn page_report_second_chance_should_return_expected() {
        let page_hit = 0;
        let frame_size = 3;
        let frame = vec![
            MemoryPage::new(4).referenced(),
            MemoryPage::new(2).referenced(),
            MemoryPage::new(3),
        ];
        let past_pages = vec![0, 4, 1, 4, 2, 4, 3, 4, 2, 4];
        let report = PageReport {
            hits: 5,
            faults: 5,
            removed: 2,
        };
        let expected_page_report = PageReport {
            hits: 5,
            faults: 6,
            removed: 3,
        };
        let (_, res) = load_page(
            algorithms::second_chance,
            frame,
            frame_size,
            page_hit,
            report,
            past_pages,
        );
        assert_eq!(res, expected_page_report);
    }
}
