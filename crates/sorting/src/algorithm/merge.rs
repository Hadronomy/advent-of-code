#[derive(Debug)]
pub struct MergeSort<'a, TContent>
where
    TContent: PartialOrd,
{
    items: &'a mut [TContent],
}

impl<'a, TContent: PartialOrd> MergeSort<'a, TContent>
where
    TContent: PartialOrd,
{
    pub fn new(input: &'a mut [TContent]) -> Self {
        Self { items: input }
    }

    pub fn sort(&mut self) {
        todo!()
    }
}
