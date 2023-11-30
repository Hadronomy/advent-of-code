use std::fmt::Debug;

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
    //! TODO: Fix this
    fn quicksort_mut(&mut self) {
        if self.len() > 1 {
            let pivot = self.hoare_partition();
            if 0 < pivot.1 {
                self[0..=pivot.1].quicksort_mut();
            }
            let end = self.len() - 1;
            if pivot.0 < end {
                self[pivot.0..=end].quicksort_mut();
            }
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
    fn hoare_partition(&mut self) -> (usize, usize);
}

impl<TContent> HoarePartition<TContent> for [TContent]
where
    TContent: Ord + Copy + Debug,
{
    fn hoare_partition(&mut self) -> (usize, usize) {
        let mut i = 0;
        let mut j = self.len() - 1;
        let pivot = self[(i + j) / 2];
        while i <= j {
            while self[i] < pivot {
                i += 1;
            }
            while self[j] > pivot {
                j -= 1;
            }
            if i <= j {
                self.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        (i, j)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort() {
        let list = vec![-1, -67, 45, 128, 27, -81, 239, 343, 99];
        let mut sorted = list.to_owned();
        sorted.quicksort_mut();
        assert_eq!(sorted, vec![-81, -67, -1, 27, 45, 99, 128, 239, 343]);
    }
}
