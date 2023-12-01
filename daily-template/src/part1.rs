#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(String::from(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn it_works() -> miette::Result<()> {
        let input = "";
        assert_eq!("example", process(input)?);
        Ok(())
    }
}
