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
            let pivot = self.hoare_partition();
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

pub trait HoarePartition<TContent>
where
    TContent: Ord + Debug,
{
    fn hoare_partition(&mut self) -> usize;
}

impl<TContent> HoarePartition<TContent> for [TContent]
where
    TContent: Ord + Copy + Debug,
{
    fn hoare_partition(&mut self) -> usize {
        let mut i = 0;
        let mut j = self.len() - 1;
        let pivot = self[(i + j) / 2];
        while i <= self.len() {
            while self[i] < pivot {
                i += 1;
            }
            while self[j] > pivot {
                j -= 1;
            }
            if i >= j {
                return j;
            }
            self.swap(i, j);
            i += 1;
            j -= 1;
        }
        j
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
