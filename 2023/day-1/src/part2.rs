#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let result = input.lines().map(process_line).sum::<u32>();
    Ok(result)
}

fn process_line(line: &str) -> u32 {
    let mut index = 0;
    let line = std::iter::from_fn(move || {
        let line_slice = &line[index..];

        let result = if line_slice.starts_with("one") {
            Some('1')
        } else if line_slice.starts_with("two") {
            Some('2')
        } else if line_slice.starts_with("three") {
            Some('3')
        } else if line_slice.starts_with("four") {
            Some('4')
        } else if line_slice.starts_with("five") {
            Some('5')
        } else if line_slice.starts_with("six") {
            Some('6')
        } else if line_slice.starts_with("seven") {
            Some('7')
        } else if line_slice.starts_with("eight") {
            Some('8')
        } else if line_slice.starts_with("nine") {
            Some('9')
        } else {
            let result = line_slice.chars().next();
            result
        };
        index += 1;
        result
    });

    let mut it = line.filter_map(|character| character.to_digit(10));
    let first = it.next().expect("should have a first digit");
    let last = it.last();
    match last {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line));
    }

    #[test]
    fn it_works() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
