// TODO: Refactor to traits
#[derive(Debug)]
pub struct InsertionSort<'a, TContent>
where
    TContent: PartialOrd,
{
    items: &'a mut [TContent],
}

impl<'a, TContent> InsertionSort<'a, TContent>
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
        for i in 1..self.items.len() {
            let mut j = i;
            while j > 0 && self.items[j - 1] > self.items[j] {
                self.items.swap(j - 1, j);
                j -= 1;
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
        InsertionSort::new(&mut list).sort();
        assert_eq!(list, vec![1, 1, 3, 4, 5, 8, 8, 9, 10]);
    }
}
