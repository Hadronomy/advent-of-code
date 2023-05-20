//! Houses the sorting algorithms used by the program

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
    pub fn run<TContent>(&self, _vector: &mut Vec<TContent>) {
        match self {
            SortingAlgorithm::Selection => todo!(),
            SortingAlgorithm::Insertion => todo!(),
            SortingAlgorithm::Merge => todo!(),
        }
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

pub trait Sorting<TContent> {
    fn sorty(&mut self, algorithm: SortingAlgorithm);
}

impl<TContent> Sorting<TContent> for Vec<TContent> {
    fn sorty(&mut self, algorithm: SortingAlgorithm) {
        algorithm.run(self);
    }
}

#[cfg(test)]
mod test {
    use crate::algorithm::{Sorting, SortingAlgorithm};

    #[test]
    fn from_str() {
        assert_eq!(
            SortingAlgorithm::try_from("selection").unwrap(),
            SortingAlgorithm::Selection
        );
        // TODO
    }

    #[test]
    fn sorty() {
        let original_vector = vec![10, 2, 30, 40, 80];
        let mut sorted_vector = original_vector.to_owned();
        sorted_vector.sorty(SortingAlgorithm::Insertion);
        assert_eq!(sorted_vector, vec![2, 10, 30, 40, 80]);
    }
}
