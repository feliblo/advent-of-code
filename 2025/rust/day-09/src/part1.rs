use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let output: i64 = input
        .lines()
        .filter_map(|line| {
            line.split_once(',').map(|(x, y)| {
                (
                    x.parse::<i64>().unwrap(),
                    y.parse::<i64>().unwrap(),
                )
            })
        })
        .tuple_combinations()
        .map(|(a, b)| axis_aligned_area(a, b))
        .max()
        .unwrap();

    Ok(output.to_string())
}

fn axis_aligned_area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let dx = (a.0 - b.0).abs() + 1;
    let dy = (a.1 - b.1).abs() + 1;
    let oppervlak = dx * dy;
    oppervlak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("50", process(input)?);
        Ok(())
    }
}
