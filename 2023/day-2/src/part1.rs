use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Vec<Cube>>,
    id: u32,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.rounds.iter().all(|round| {
            round.iter().all(|cube| match cube {
                Cube::Red(count) => *count <= 12,
                Cube::Green(count) => *count <= 13,
                Cube::Blue(count) => *count <= 14,
            })
        })
    }
}

fn cube_parser(input: &str) -> IResult<&str, Cube> {
    let (input, (num, color)) = separated_pair(
        complete::u32,
        tag(" "),
        map(alpha1, |color| match color {
            "red" => Cube::Red,
            "green" => Cube::Green,
            "blue" => Cube::Blue,
            _ => panic!("unknown color: {}", color),
        }),
    )(input)?;
    Ok((input, color(num)))
}

fn round_parser(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube_parser)(input)?;
    Ok((input, cubes))
}

fn game_parser(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round_parser))(input)?;
    Ok((input, Game { rounds, id }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game_parser)(input)?;
    Ok((input, games))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_, games) = parse_games(input).expect("should parse games");
    let result = games
        .iter()
        .filter_map(|game| if game.is_valid() { Some(game.id) } else { None })
        .sum::<u32>();
    // dbg!(games);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn it_works() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
