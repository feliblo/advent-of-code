use std::collections::HashMap;
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

    // Map from column position to count of timelines at that position
    let mut timeline_counts: HashMap<usize, usize> = HashMap::new();
    timeline_counts.insert(start, 1);

    for row in &lines[1..] {
        let mut next_counts: HashMap<usize, usize> = HashMap::new();

        for (&col, &count) in &timeline_counts {
            let children = next_beams_for_cell(col, row, width);
            for child_col in children {
                *next_counts.entry(child_col).or_insert(0) += count;
            }
        }

        timeline_counts = next_counts;
    }

    // Sum all timeline counts at the end
    let total: usize = timeline_counts.values().sum();
    Ok(total.to_string())
}
fn next_beams_for_cell(
    col: usize,
    row: &str,
    width: usize,
) -> Vec<usize> {
    let cell = row.as_bytes()[col] as char;

    if cell == SPLITTER {
        let mut children = Vec::new();
        // Left path
        if col > 0 {
            let left_col = col - 1;
            if left_col < width {
                children.push(left_col);
            }
        }
        // Right path
        if col + 1 < width {
            children.push(col + 1);
        }
        children
    } else {
        vec![col] // beam continues downward
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
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
