// TODO: Refactor to traits
#[derive(Debug)]
pub struct SelectionSort<'a, TContent>
where
    TContent: PartialOrd,
{
    items: &'a mut [TContent],
}

impl<'a, TContent> SelectionSort<'a, TContent>
where
    TContent: PartialOrd,
{
    pub fn new(input: &'a mut [TContent]) -> Self {
        Self { items: input }
    }

    pub fn sort(&mut self) {
        if self.items.len() < 2 {
            return;
        }
        for i in 0..self.items.len() {
            let mut min_index = i;
            for j in i + 1..self.items.len() {
                if self.items[j] < self.items[min_index] {
                    min_index = j;
                }
            }
            if i != min_index {
                self.items.swap(i, min_index);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort() {
        let mut list = vec![10, 1, 8, 4, 5, 3, 9, 1, 8];
        SelectionSort::new(&mut list).sort();
        assert_eq!(list, vec![1, 1, 3, 4, 5, 8, 8, 9, 10]);
    }
}
