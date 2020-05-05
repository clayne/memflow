pub mod page_chunks;
pub use page_chunks::*;

#[cfg(test)]
mod tests {
    use crate::page_chunks::{PageChunks, PageChunksMut};

    const PAGE_SIZE: usize = 97;
    const OFF: usize = 26;
    const ADDEND: usize = 17;

    #[test]
    fn pc_check_all_aligned_zero() {
        let arr = [0_u8; 0x1000];

        for (addr, _chunk) in PageChunks::create_from(&arr, 0.into(), PAGE_SIZE.into()) {
            assert_eq!(addr.as_page_aligned(PAGE_SIZE.into()), addr);
        }
    }

    #[test]
    fn pc_check_all_chunks_equal() {
        let arr = [0_u8; 100 * PAGE_SIZE];

        for (_addr, chunk) in PageChunks::create_from(&arr, 0.into(), PAGE_SIZE.into()) {
            println!("{:x} {:x}", _addr, chunk.len());
            assert_eq!(chunk.len(), PAGE_SIZE);
        }
    }

    #[test]
    fn pc_check_all_chunks_equal_first_not() {
        const OFF: usize = 26;
        let arr = [0_u8; 100 * PAGE_SIZE + (PAGE_SIZE - OFF)];

        let mut page_iter = PageChunks::create_from(&arr, OFF.into(), PAGE_SIZE.into());

        {
            let (addr, chunk) = page_iter.next().unwrap();
            assert_eq!(addr, OFF.into());
            assert_eq!(chunk.len(), PAGE_SIZE - OFF);
        }

        for (_addr, chunk) in page_iter {
            assert_eq!(chunk.len(), PAGE_SIZE);
        }
    }

    #[test]
    fn pc_check_everything() {
        const TOTAL_LEN: usize = 100 * PAGE_SIZE + ADDEND - OFF;
        let arr = [0_u8; TOTAL_LEN];

        let mut cur_len = 0;
        let mut prev_len = 0;

        let mut page_iter = PageChunks::create_from(&arr, OFF.into(), PAGE_SIZE.into());

        {
            let (addr, chunk) = page_iter.next().unwrap();
            assert_eq!(addr, OFF.into());
            assert_eq!(chunk.len(), PAGE_SIZE - OFF);
            cur_len += chunk.len();
        }

        for (_addr, chunk) in page_iter {
            if chunk.len() != ADDEND {
                assert_eq!(chunk.len(), PAGE_SIZE);
            }
            prev_len = chunk.len();
            cur_len += prev_len;
        }

        assert_eq!(prev_len, ADDEND);
        assert_eq!(cur_len, TOTAL_LEN);
    }

    #[test]
    fn pc_check_size_hint() {
        const PAGE_COUNT: usize = 5;
        let arr = [0_u8; PAGE_SIZE * PAGE_COUNT];
        assert_eq!(
            PageChunks::create_from(&arr, 0.into(), PAGE_SIZE.into())
                .size_hint()
                .0,
            PAGE_COUNT
        );
        assert_eq!(
            PageChunks::create_from(&arr, 1.into(), PAGE_SIZE.into())
                .size_hint()
                .0,
            PAGE_COUNT + 1
        );
        assert_eq!(
            PageChunks::create_from(&arr, (PAGE_SIZE - 1).into(), PAGE_SIZE.into())
                .size_hint()
                .0,
            PAGE_COUNT + 1
        );
        assert_eq!(
            PageChunks::create_from(&arr, PAGE_SIZE.into(), PAGE_SIZE.into())
                .size_hint()
                .0,
            PAGE_COUNT
        );
    }

    #[test]
    fn pc_mut_check_all_aligned_zero() {
        let mut arr = [0_u8; 0x1000];

        for (addr, _chunk) in PageChunksMut::create_from(&mut arr, 0.into(), PAGE_SIZE.into()) {
            assert_eq!(addr.as_page_aligned(PAGE_SIZE.into()), addr);
        }
    }

    #[test]
    fn pc_mut_check_all_chunks_equal() {
        let mut arr = [0_u8; 100 * PAGE_SIZE];

        for (_addr, chunk) in PageChunksMut::create_from(&mut arr, 0.into(), PAGE_SIZE.into()) {
            println!("{:x} {:x}", _addr, chunk.len());
            assert_eq!(chunk.len(), PAGE_SIZE);
        }
    }

    #[test]
    fn pc_mut_check_all_chunks_equal_first_not() {
        const OFF: usize = 26;
        let mut arr = [0_u8; 100 * PAGE_SIZE + (PAGE_SIZE - OFF)];

        let mut page_iter = PageChunksMut::create_from(&mut arr, OFF.into(), PAGE_SIZE.into());

        {
            let (addr, chunk) = page_iter.next().unwrap();
            assert_eq!(addr, OFF.into());
            assert_eq!(chunk.len(), PAGE_SIZE - OFF);
        }

        for (_addr, chunk) in page_iter {
            assert_eq!(chunk.len(), PAGE_SIZE);
        }
    }

    #[test]
    fn pc_mut_check_everything() {
        const TOTAL_LEN: usize = 100 * PAGE_SIZE + ADDEND - OFF;
        let mut arr = [0_u8; TOTAL_LEN];

        let mut cur_len = 0;
        let mut prev_len = 0;

        let mut page_iter = PageChunksMut::create_from(&mut arr, OFF.into(), PAGE_SIZE.into());

        {
            let (addr, chunk) = page_iter.next().unwrap();
            assert_eq!(addr, OFF.into());
            assert_eq!(chunk.len(), PAGE_SIZE - OFF);
            cur_len += chunk.len();
        }

        for (_addr, chunk) in page_iter {
            if chunk.len() != ADDEND {
                assert_eq!(chunk.len(), PAGE_SIZE);
            }
            prev_len = chunk.len();
            cur_len += prev_len;
        }

        assert_eq!(prev_len, ADDEND);
        assert_eq!(cur_len, TOTAL_LEN);
    }
}
