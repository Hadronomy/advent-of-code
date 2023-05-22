//! Houses the sorting algorithms used by the program

use self::{insertion::InsertionSort, merge::MergeSort, selection::SelectionSort};

mod insertion;
mod merge;
mod selection;

/// Sorting Algorithms
#[derive(Debug, PartialEq)]
pub enum SortingAlgorithm {
    Selection,
    Insertion,
    Merge,
}

impl SortingAlgorithm {
    /// Runs the specified [`SortingAlgorithm`]
    pub fn run<TContent>(&self, vector: &mut [TContent])
    where
        TContent: PartialOrd,
    {
        match self {
            SortingAlgorithm::Selection => SelectionSort::<TContent>::new(vector).sort(),
            SortingAlgorithm::Insertion => InsertionSort::<TContent>::new(vector).sort(),
            SortingAlgorithm::Merge => MergeSort::<TContent>::new(vector).sort(),
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
            _ => Err("Invalid input"),
        }
    }
}

pub trait Sorting<TContent>
where
    TContent: PartialOrd,
{
    fn sorty(&mut self, algorithm: SortingAlgorithm);
}

impl<TContent> Sorting<TContent> for [TContent]
where
    TContent: PartialOrd,
{
    fn sorty(&mut self, algorithm: SortingAlgorithm) {
        algorithm.run(self);
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
