use crate::*;
use utils::{add_if_not_found, push, remove_first};

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
/// Keep sort order of [Oldest -> Newest]
pub fn fifo(
    page_frames: Vec<MemoryPage>,
    page: MemoryPage,
    _past_pages: Vec<u32>,
) -> Vec<MemoryPage> {
    push(remove_first(page_frames), page)
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
/// No Sort Order
pub fn lru(
    page_frames: Vec<MemoryPage>,
    page: MemoryPage,
    mut past_pages: Vec<u32>,
) -> Vec<MemoryPage> {
    let length = page_frames.len();
    past_pages.reverse();
    let filter_pages = past_pages
        .into_iter()
        .fold(Vec::<u32>::new(), add_if_not_found);

    let replace_page_number = *filter_pages.get(length - 1).unwrap();

    let page_frames = page_frames
        .into_iter()
        .filter(|x| x.number != replace_page_number)
        .collect::<Vec<MemoryPage>>();
    push(page_frames, page)
}

/// Second Chance Algorithm
/// Exactly like FIFO and that it starts to replace the oldest page
/// However, if the oldest has been referenced, then clear it, and look at the second oldest page
/// Continue looking at the next oldest till you find one that has not been referenced and replace it
/// Keep sort order of [Oldest -> Newest]
pub fn second_chance(
    page_frames: Vec<MemoryPage>,
    page: MemoryPage,
    mut _past_pages: Vec<u32>,
) -> Vec<MemoryPage> {
    recursive_second_chance(page_frames, page)
}

pub fn recursive_second_chance(
    mut page_frames: Vec<MemoryPage>,
    page: MemoryPage,
) -> Vec<MemoryPage> {
    let oldest_page = page_frames.get(0).unwrap();
    // Base Case
    if oldest_page.referenced == false {
        // replace it
        page_frames.remove(0);
        page_frames.push(page);
        return page_frames;
    }

    page_frames.push(oldest_page.clear());
    page_frames.remove(0);

    recursive_second_chance(page_frames, page)
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

        let expected = vec![
            MemoryPage::new(2),
            MemoryPage::new(0),
            MemoryPage::new(3),
            MemoryPage::new(4),
        ];

        let res = lru(page_frames, page, past_pages);
        assert_eq!(res, expected);
    }

    #[test]
    fn fifo_should_return_expected() {
        let page_frames = vec![MemoryPage::new(0), MemoryPage::new(3), MemoryPage::new(5)];
        let page = MemoryPage::new(6);
        let past_pages = vec![];
        let expected = vec![MemoryPage::new(3), MemoryPage::new(5), MemoryPage::new(6)];

        let res = fifo(page_frames, page, past_pages);
        assert_eq!(res, expected);
    }

    #[test]
    fn second_chance_all_frame_no_second_chance_should_return_expected() {
        let page_frames = vec![MemoryPage::new(0), MemoryPage::new(1), MemoryPage::new(2)];
        let page = MemoryPage::new(3);
        let expected = vec![MemoryPage::new(1), MemoryPage::new(2), MemoryPage::new(3)];

        let res = second_chance(page_frames, page, vec![]);
        assert_eq!(res, expected);
    }

    #[test]
    fn second_chance_oldest_page_referenced_should_replace_second_oldest() {
        let page_frames = vec![
            MemoryPage::new(0).referenced(),
            MemoryPage::new(1),
            MemoryPage::new(2),
        ];
        let page = MemoryPage::new(3);
        let expected = vec![MemoryPage::new(2), MemoryPage::new(0), MemoryPage::new(3)];
        let res = second_chance(page_frames, page, vec![]);
        assert_eq!(res, expected);
    }

    #[test]
    fn second_chance_two_oldest_pages_referenced_should_replace_third_oldest() {
        let page_frames = vec![
            MemoryPage::new(0).referenced(),
            MemoryPage::new(1).referenced(),
            MemoryPage::new(2),
        ];
        let page = MemoryPage::new(3);
        let expected = vec![MemoryPage::new(0), MemoryPage::new(1), MemoryPage::new(3)];
        let res = second_chance(page_frames, page, vec![]);
        assert_eq!(res, expected);
    }

    #[test]
    fn second_chance_all_pages_referenced_should_resort_to_replace_oldest() {
        let page_frames = vec![
            MemoryPage::new(0).referenced(),
            MemoryPage::new(1).referenced(),
            MemoryPage::new(2).referenced(),
        ];
        let page = MemoryPage::new(3);
        let expected = vec![MemoryPage::new(1), MemoryPage::new(2), MemoryPage::new(3)];
        let res = second_chance(page_frames, page, vec![]);
        assert_eq!(res, expected);
    }
}
