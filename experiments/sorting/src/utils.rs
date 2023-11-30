use std::slice::from_raw_parts_mut;

pub trait Split<TContent> {
    fn split_at_mut_exclusive(&mut self, mid: usize) -> (&mut [TContent], &mut [TContent]);
}

impl<TContent> Split<TContent> for [TContent] {
    fn split_at_mut_exclusive(&mut self, mid: usize) -> (&mut [TContent], &mut [TContent]) {
        let len = self.len();
        let ptr = self.as_mut_ptr();

        unsafe {
            assert!(mid <= len);
            (
                from_raw_parts_mut(ptr, mid),
                from_raw_parts_mut(ptr.add(mid + 1), len - mid - 1),
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_at_mut_exclusive() {
        let mut list = vec![1, 2, 3, 4, 5, 6, 7];
        let mid = 3;
        let (left, right) = list.split_at_mut_exclusive(mid);
        assert_eq!(left, vec![1, 2, 3]);
        assert_eq!(right, vec![5, 6, 7]);
    }
}
