use itertools::Itertools;
use tracing::*;

const ENTRY_POINT: char = 'S';
const SPLITTER: char = '^';

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();

    let start = lines[0]
        .chars()
        .position(|c| c == ENTRY_POINT)
        .expect("no S in first line");

    let mut beams = vec![start];
    let mut split_count = 0;

    for row in 1..lines.len() {
        let mut next_beams = Vec::new();

        for &col in &beams {
            let cell_value =
                lines[row].as_bytes()[col] as char;

            if cell_value == SPLITTER {
                split_count += 1;
                if col > 0 {
                    next_beams.push(col - 1);
                }
                if col + 1 < width {
                    next_beams.push(col + 1);
                }
            } else {
                next_beams.push(col);
            }
        }
        beams = next_beams.into_iter().unique().collect();
    }

    Ok(split_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
