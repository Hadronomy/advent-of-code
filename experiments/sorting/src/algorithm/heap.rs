use std::fmt::Debug;

pub trait HeapSort<TContent>
where
    TContent: Ord + Debug,
{
    fn heap_max(&mut self, i: usize, heap_len: usize);
    fn heap_sort_mut(&mut self);
}

impl<TContent> HeapSort<TContent> for [TContent]
where
    TContent: Ord + Debug,
{
    fn heap_max(&mut self, i: usize, heap_len: usize) {
        let left = 2 * i + 1;
        let right = left + 1;
        let mut largest = i;
        if left < heap_len && self[left] > self[largest] {
            largest = left;
        }
        if right < heap_len && self[right] > self[largest] {
            largest = right;
        }
        if largest != i {
            self.swap(i, largest);
            self.heap_max(largest, heap_len);
        }
    }

    fn heap_sort_mut(&mut self) {
        if self.len() < 2 {
            return;
        }
        for i in (0..self.len() / 2).rev() {
            self.heap_max(i, self.len());
        }
        for i in (0..self.len()).rev() {
            self.swap(0, i);
            self.heap_max(0, i);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort() {
        let mut list = vec![10, 1, 8, 4, 5, 3, 9, 1, 8];
        list.heap_sort_mut();
        assert_eq!(list, vec![1, 1, 3, 4, 5, 8, 8, 9, 10]);
    }
}
