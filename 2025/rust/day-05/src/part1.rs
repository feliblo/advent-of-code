use miette::{IntoDiagnostic, Result};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut lines = _input.lines();

    let fresh_ingredient_ranges: Result<Vec<(i64, i64)>> = lines
    .by_ref()
    .take_while(|l| !l.is_empty())
    .map(|l| {
        let (start, end) = l
            .split_once('-')
            .ok_or_else(|| miette::miette!("Invalid range, missing '-' in: {}", l))?;

        Ok((
            start.trim().parse::<i64>().into_diagnostic()?,
            end.trim().parse::<i64>().into_diagnostic()?,
        ))
    })
    .collect();

    if let Ok(ingredient_ranges) = fresh_ingredient_ranges {
        let ingredients_are_fresh: i64 =
            lines.fold(0, |acc, l| {
                let ingredient_number =
                    l.trim().parse::<i64>().expect(
                        "failed parsing but shouldn't have",
                    );

                if ingredient_ranges.iter().any(
                    |&(start, end)| {
                        (start..=end)
                            .contains(&ingredient_number)
                    },
                ) {
                    acc + 1
                } else {
                    acc
                }
            });
        return Ok(ingredients_are_fresh.to_string());
    }
    Ok("wrong".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
