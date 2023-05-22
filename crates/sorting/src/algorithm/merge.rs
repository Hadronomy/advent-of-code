#[derive(Debug)]
pub struct MergeSort<'a, TContent>
where
    TContent: PartialOrd,
{
    vector: &'a [TContent],
}

impl<'a, TContent: PartialOrd> MergeSort<'a, TContent>
where
    TContent: PartialOrd,
{
    pub fn new(input: &'a [TContent]) -> Self {
        Self { vector: input }
    }

    pub fn sort(&mut self) {}
}
