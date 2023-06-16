use std::fmt::Debug;

use crate::utils::Split;

pub trait QuickSort<TContent>
where
    TContent: Ord + Debug,
{
    fn quicksort_mut(&mut self);
}

impl<TContent> QuickSort<TContent> for [TContent]
where
    TContent: Ord + Copy + Debug,
{
    fn quicksort_mut(&mut self) {
        if self.len() > 1 {
            let pivot = self.lomuto_partition();
            let (left, right) = self.split_at_mut_exclusive(pivot);
            left.quicksort_mut();
            right.quicksort_mut();
        }
    }
}

pub trait LomutoPartition<TContent>
where
    TContent: Ord + Debug,
{
    fn lomuto_partition(&mut self) -> usize;
}

impl<TContent> LomutoPartition<TContent> for [TContent]
where
    TContent: Ord + Copy + Debug,
{
    fn lomuto_partition(&mut self) -> usize {
        let pivot = self.len() - 1;
        let mut swap = 0;
        for i in 0..pivot {
            if self[i] < self[pivot] {
                if swap != i {
                    self.swap(swap, i);
                }
                swap += 1;
            }
        }
        if swap != pivot {
            self.swap(swap, pivot);
        }
        swap
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort() {
        let list = vec![10, 1, 8, 4, 5, 3, 9, 1, 8];
        let mut sorted = list.to_owned();
        sorted.quicksort_mut();
        assert_eq!(sorted, vec![1, 1, 3, 4, 5, 8, 8, 9, 10]);
    }
}
