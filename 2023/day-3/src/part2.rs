use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::digit1,
    combinator::iterator,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Value<'a> {
    Empty,
    Symbol(SpanIVec2<'a>),
    Number(SpanIVec2<'a>),
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(input: Span) -> IResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1.map(with_xy).map(Value::Number),
            tag("*").map(with_xy).map(Value::Symbol),
            take_till1(|c: char| c.is_ascii_digit() || c == '*').map(|_| Value::Empty),
        )),
    );
    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    let res = it.finish();
    res.map(|(input, _)| (input, parsed))
}

const POSITIONS: [IVec2; 8] = [
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let grid = parse_grid(Span::new(input)).unwrap().1;
    let number_map = grid
        .iter()
        .filter_map(|value| match value {
            Value::Empty => None,
            Value::Symbol(_) => None,
            Value::Number(num) => Some((
                num.extra,
                num.fragment(),
                // used as ID
                num.location_offset(),
            )),
        })
        .flat_map(|(ivec, fragment, id)| {
            (ivec.x..(ivec.x + fragment.len() as i32))
                .map(move |x| (IVec2::new(x, ivec.y), (id, fragment)))
        })
        .collect::<HashMap<IVec2, (usize, &&str)>>();

    let result = grid
        .iter()
        .filter_map(|value| {
            let Value::Symbol(symbol) = value else {
                return None;
            };
            let matching_numbers = POSITIONS
                .iter()
                .map(|pos| *pos + symbol.extra)
                .filter_map(|surrounding_symbol_pos| number_map.get(&surrounding_symbol_pos))
                .unique()
                .map(|(_, fragment)| fragment.parse::<i32>().expect("should be a valid number"))
                .collect::<Vec<i32>>();
            (matching_numbers.len() == 2).then_some(matching_numbers.iter().product::<i32>())
        })
        .sum::<i32>();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn it_works() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}
