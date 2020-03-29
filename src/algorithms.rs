use crate::*;
use utils::add_if_not_found;

/*
* All algorithms should return back the page *number* to replace. Not the index.
*/

/// First In First Out Algorithm
/// page_frames is the currently loaded pages in memory
/// page is the page we want to load into memory
/// past_pages is the pages that have been loaded before this current page
///
/// Select the very first page to have enter the frame for replacement
///
/// Being selected again does not mean you entered the queue again
/// For Example: [0, 1, 2, 0]
///     By FIFO '0' is still the very first to have entered the queue
///
pub fn fifo(page_frames: Vec<MemoryPage>, _page: MemoryPage, past_pages: Vec<u32>) -> u32 {
    let length = page_frames.len();
    let filter_pages: Vec<u32> = past_pages
        .into_iter()
        .fold(Vec::<u32>::new(), add_if_not_found);

    *filter_pages.as_slice()[filter_pages.len() - length..]
        .first()
        .unwrap()
}

/// Least Recently Used Algorithm
/// page_frames is the currently loaded pages in memory
/// page is the page we want to load into memory
/// past_pages is the pages that have been loaded before this current page
///
/// Select the oldest page in the frame that hasn't been used recently
///
/// For Example: [0, 1, 2, 0]
///     By LRU: '1' is the oldest page in the queue
///
pub fn lru(page_frames: Vec<MemoryPage>, _page: MemoryPage, mut past_pages: Vec<u32>) -> u32 {
    let length = page_frames.len();
    past_pages.reverse();
    let filter_pages = past_pages
        .into_iter()
        .fold(Vec::<u32>::new(), add_if_not_found);

    *filter_pages.get(length - 1).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lru_should_return_expected() {
        let page_frames = vec![
            MemoryPage::new(2),
            MemoryPage::new(1),
            MemoryPage::new(0),
            MemoryPage::new(3),
        ];
        let page = MemoryPage::new(4);
        let past_pages = vec![7, 0, 1, 2, 0, 3, 0];
        let expected_page_number = 1;

        let res = lru(page_frames, page, past_pages);
        assert_eq!(res, expected_page_number);
    }

    #[test]
    fn fifo_should_return_expected() {
        let page_frames = vec![MemoryPage::new(0), MemoryPage::new(3), MemoryPage::new(5)];
        let page = MemoryPage::new(6);
        let past_pages = vec![1, 3, 0, 3, 5];
        let expected_page_number = 3;

        let res = fifo(page_frames, page, past_pages);
        assert_eq!(res, expected_page_number);
    }

    #[test]
    fn fifo_case2_should_return_expected() {
        let page_frames = vec![MemoryPage::new(0), MemoryPage::new(1), MemoryPage::new(2)];
        let page = MemoryPage::new(3);
        let past_pages = vec![0, 1, 2, 0];
        let expected_page_number = 0;

        let res = fifo(page_frames, page, past_pages);
        assert_eq!(res, expected_page_number);
    }
}
