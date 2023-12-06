use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse_times(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(numbers, line_ending, numbers).parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let (_, (times, distances)) = parse_times(input).expect("a valid parse");
    let result = times
        .into_iter()
        .zip(distances)
        .map(|(time, record_distance)| {
            (0..time)
                .filter_map(|speed| {
                    let distance = (time - speed) * speed;
                    (distance > record_distance).then_some(distance)
                })
                .count()
        })
        .product::<usize>();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn it_works() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
