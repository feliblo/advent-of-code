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

    for row in &lines[1..] {
        let mut next_beams = Vec::new();

        for &col in &beams {
            let (children, split) =
                next_beams_for_cell(col, row, width);
            if split {
                split_count += 1;
            }
            next_beams.extend(children);
        }

        beams = next_beams.into_iter().unique().collect();
    }

    Ok(split_count.to_string())
}
fn next_beams_for_cell(
    col: usize,
    row: &str,
    width: usize,
) -> (Vec<usize>, bool) {
    let cell = row.as_bytes()[col] as char;

    if cell == SPLITTER {
        let mut children = Vec::new();
        if col > 0 {
            children.push(col - 1);
        }
        if col + 1 < width {
            children.push(col + 1);
        }
        (children, true) // true = beam split
    } else {
        (vec![col], false) // beam continues downward
    }
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
