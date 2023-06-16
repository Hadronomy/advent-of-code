//! Mergesort implementation
//!
//! Based of @gvelim implementation of merge sort in
//! https://github.com/gvelim/CSX0003RUST

use std::fmt::Debug;

/// Simple MergeSort implementation
pub trait MergeSort<TContent>
where
    TContent: Ord,
{
    fn mergesort_mut<FMerge>(&mut self, fn_merge: FMerge)
    where
        FMerge: Copy + FnMut(&mut [TContent], &mut [TContent]);
}

impl<TContent> MergeSort<TContent> for [TContent]
where
    TContent: Ord + Copy + Clone + Debug,
{
    fn mergesort_mut<FMerge>(&mut self, mut fn_merge: FMerge)
    where
        FMerge: Copy + FnMut(&mut [TContent], &mut [TContent]),
    {
        let len = self.len();
        match len {
            0..=1 => (),
            2 => {
                if self[0] > self[1] {
                    self.swap(0, 1);
                }
            }
            _ => {
                let (left, right) = self.split_at_mut(len >> 1);
                left.mergesort_mut(fn_merge);
                right.mergesort_mut(fn_merge);
                fn_merge(left, right);
            }
        }
    }
}

pub trait Merge<TContent>
where
    TContent: Ord + Debug,
{
    fn merge_mut(&mut self, right: &mut [TContent]);
}

impl<TContent> Merge<TContent> for [TContent]
where
    TContent: Ord + Copy + Debug,
{
    fn merge_mut(&mut self, right: &mut [TContent]) {
        let mut i = 0;
        let mut j = 0;
        let mut temp = Vec::<TContent>::new();

        loop {
            match (i < self.len(), j < right.len()) {
                (true, false) => {
                    temp.push(self[i]);
                    i += 1;
                }
                (true, true) if self[i] <= right[j] => {
                    temp.push(self[i]);
                    i += 1;
                }
                (_, true) => {
                    temp.push(right[j]);
                    j += 1;
                }
                (_, _) => break,
            }
        }

        self.copy_from_slice(&temp[0..self.len()]);
        for temp_index in self.len()..temp.len() {
            right[temp_index - self.len()] = temp[temp_index];
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort() {
        let list = vec![10, 1, 8, 4, 5, 3, 9, 1, 8];
        let mut sorted = list.to_owned();
        sorted.mergesort_mut(Merge::merge_mut);
        assert_eq!(sorted, vec![1, 1, 3, 4, 5, 8, 8, 9, 10]);
    }
}
