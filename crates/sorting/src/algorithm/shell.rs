use std::fmt::Debug;

pub trait ShellSort<TContent>
where
    TContent: Copy + Ord + Debug,
{
    fn shell_sort_mut(&mut self);
}

impl<TContent> ShellSort<TContent> for [TContent]
where
    TContent: Copy + Ord + Debug,
{
    fn shell_sort_mut(&mut self) {
        let mut delta = self.len() >> 1;
        while delta > 0 {
            for start in 0..delta {
                insertion(self, start, delta);
            }
            delta /= 2;
        }
    }
}

fn insertion<TContent>(values: &mut [TContent], start: usize, delta: usize)
where
    TContent: Copy + Ord + Debug,
{
    for i in ((start + delta)..values.len()).step_by(delta) {
        let current = values[i];
        let mut position = i;
        while position >= delta && values[position - delta] > current {
            values[position] = values[position - delta];
            position -= delta;
        }
        values[position] = current;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort() {
        let list = vec![10, 1, 8, 4, 5, 3, 9, 1, 8];
        let mut sorted = list.to_owned();
        sorted.shell_sort_mut();
        assert_eq!(sorted, vec![1, 1, 3, 4, 5, 8, 8, 9, 10]);
    }
}
