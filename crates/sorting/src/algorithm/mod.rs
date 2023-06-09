//! Houses the sorting algorithms used by the program
use std::fmt::Debug;

use self::{
    heap::HeapSort,
    insertion::InsertionSort,
    merge::{Merge, MergeSort},
    quick::QuickSort,
    selection::SelectionSort,
    shell::ShellSort,
};

pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod radix;
pub mod selection;
pub mod shell;

/// Sorting Algorithms
#[derive(Debug, PartialEq)]
pub enum SortingAlgorithm {
    Selection,
    Insertion,
    Merge,
    Quick,
    Heap,
    // Radix,
    Shell,
}

impl SortingAlgorithm {
    /// Runs the specified [`SortingAlgorithm`]
    pub fn sort_mut<TContent>(&self, vector: &mut [TContent])
    where
        TContent: Ord + Copy + Clone + Debug,
    {
        match self {
            SortingAlgorithm::Selection => SelectionSort::<TContent>::new(vector).sort(),
            SortingAlgorithm::Insertion => InsertionSort::<TContent>::new(vector).sort(),
            SortingAlgorithm::Merge => vector.mergesort_mut(Merge::merge_mut),
            SortingAlgorithm::Quick => vector.quicksort_mut(),
            SortingAlgorithm::Heap => vector.heap_sort_mut(),
            // SortingAlgorithm::Radix => vector.radix_sort_mut(),
            SortingAlgorithm::Shell => vector.shell_sort_mut(),
        };
    }
}

impl TryFrom<&'static str> for SortingAlgorithm {
    type Error = &'static str;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        match value {
            "selection" => Ok(SortingAlgorithm::Selection),
            "insertion" => Ok(SortingAlgorithm::Insertion),
            "merge" => Ok(SortingAlgorithm::Merge),
            "quick" => Ok(SortingAlgorithm::Quick),
            "heap" => Ok(SortingAlgorithm::Heap),
            // "radix" => Ok(SortingAlgorithm::Radix),
            "shell" => Ok(SortingAlgorithm::Shell),
            _ => Err("Invalid input"),
        }
    }
}

pub trait Sorting<TContent>
where
    TContent: Ord + Copy + Clone + Debug,
{
    fn sorty(&mut self, algorithm: SortingAlgorithm);
}

impl<TContent> Sorting<TContent> for [TContent]
where
    TContent: Ord + Copy + Clone + Debug,
{
    fn sorty(&mut self, algorithm: SortingAlgorithm) {
        algorithm.sort_mut(self);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(
            SortingAlgorithm::try_from("selection").unwrap(),
            SortingAlgorithm::Selection
        );
        assert_eq!(
            SortingAlgorithm::try_from("insertion").unwrap(),
            SortingAlgorithm::Insertion
        );
        assert_eq!(
            SortingAlgorithm::try_from("merge").unwrap(),
            SortingAlgorithm::Merge
        )
    }

    #[test]
    fn sorty() {
        let mut list = vec![10, 2, 30, 40, 80];
        list.sorty(SortingAlgorithm::Insertion);
        assert_eq!(list, vec![2, 10, 30, 40, 80]);
    }
}
