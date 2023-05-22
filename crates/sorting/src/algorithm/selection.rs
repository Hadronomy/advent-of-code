#[derive(Debug)]
pub struct SelectionSort<'a, TContent>
where
    TContent: PartialOrd,
{
    items: &'a [TContent],
}

impl<'a, TContent> SelectionSort<'a, TContent>
where
    TContent: PartialOrd,
{
    pub fn new(input: &'a [TContent]) -> Self {
        Self { items: input }
    }

    pub fn sort(&mut self) {
        todo!();
    }
}
