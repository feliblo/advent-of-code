use miette::{IntoDiagnostic, Result};

#[tracing::instrument]
pub fn process(_input: &str) -> Result<String> {
    let mut ingredient_ranges: Vec<(i64, i64)> = _input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let (s, e) =
                line.split_once('-').ok_or_else(|| {
                    miette::miette!(
                        "Invalid range: {}",
                        line
                    )
                })?;
            Ok((
                s.trim()
                    .parse::<i64>()
                    .into_diagnostic()?,
                e.trim()
                    .parse::<i64>()
                    .into_diagnostic()?,
            ))
        })
        .collect::<Result<_>>()?;

    ingredient_ranges.sort_by_key(|&(start, _)| start);

    let mut extended_intervals: Vec<(i64, i64)> =
        Vec::new();
    for (start, end) in ingredient_ranges {
        if let Some((_, last_end)) =
            extended_intervals.last_mut()
        {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                extended_intervals.push((start, end));
            }
        } else {
            extended_intervals.push((start, end));
        }
    }

    let total: i64 = extended_intervals
        .iter()
        .map(|(s, e)| e - s + 1)
        .sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
