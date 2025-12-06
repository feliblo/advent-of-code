use std::ops::{Add, Mul};

enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn from_char(c: char) -> Self {
        match c {
            '*' => Self::Multiply,
            '+' => Self::Add,
            _ => panic!("Invalid operator: {}", c),
        }
    }

    fn apply(&self, values: Vec<i64>) -> i64 {
        match self {
            Self::Add => {
                values.into_iter().reduce(Add::add).unwrap()
            }
            Self::Multiply => {
                values.into_iter().reduce(Mul::mul).unwrap()
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines: Vec<&[u8]> =
        input.lines().map(str::as_bytes).collect();
    let operators = lines.last().unwrap();
    let grid = &lines[..lines.len() - 1];
    let width =
        lines.iter().map(|l| l.len()).max().unwrap();

    let mut total = 0;
    let mut col = 0;

    while col < width {
        if is_empty_column(grid, col) {
            col += 1;
            continue;
        }

        let start = col;
        let mut end = col;
        while end < width && !is_empty_column(grid, end) {
            end += 1;
        }

        let mut values = Vec::new();
        for c in (start..end).rev() {
            let mut digits = Vec::new();
            for &row in grid.iter() {
                if c < row.len() && row[c] != b' ' {
                    digits.push(row[c]);
                }
            }

            if !digits.is_empty() {
                let mut value: i64 = 0;
                for &digit in &digits {
                    value =
                        value * 10 + (digit - b'0') as i64;
                }
                values.push(value);
            }
        }

        let op = if start < operators.len() {
            Operator::from_char(operators[start] as char)
        } else {
            Operator::Add
        };

        total += op.apply(values);
        col = end;
    }

    Ok(total.to_string())
}

fn is_empty_column(grid: &[&[u8]], col: usize) -> bool {
    grid.iter()
        .all(|row| col >= row.len() || row[col] == b' ')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";
        assert_eq!("3263827", process(input)?);
        Ok(())
    }
}
