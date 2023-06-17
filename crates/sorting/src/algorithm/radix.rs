use num_traits::{PrimInt, Unsigned};
use std::fmt::Debug;

pub trait RadixSort<TContent>
where
    TContent: Ord + Debug,
{
    fn radix_sort_mut(&mut self);
}

impl<TContent> RadixSort<TContent> for [TContent]
where
    TContent: PrimInt + Unsigned + Debug,
{
    fn radix_sort_mut(&mut self) {
        let max = match self.iter().max() {
            Some(&value) => value,
            None => return,
        };
        // let radix = TContent::from(self.len().next_power_of_two()).unwrap();
        let radix = TContent::from(10).unwrap();
        let mut place = TContent::one();
        while max >= place {
            let digit_of = |x: TContent| (x / place % radix).to_usize().unwrap();
            let mut counter = vec![0; radix.to_usize().unwrap()];
            for &x in self.iter() {
                counter[digit_of(x)] += 1;
            }
            for i in 1..radix.to_usize().unwrap() {
                counter[i] += counter[i - 1];
            }
            for &x in self.to_owned().iter().rev() {
                counter[digit_of(x)] -= 1;
                self[counter[digit_of(x)]] = x;
            }
            place = place * radix;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort() {
        let list: Vec<u32> = vec![10, 1, 8, 4, 5, 3, 9, 1, 8];
        let mut sorted = list.to_owned();
        sorted.radix_sort_mut();
        assert_eq!(sorted, vec![1, 1, 3, 4, 5, 8, 8, 9, 10]);
    }
}
