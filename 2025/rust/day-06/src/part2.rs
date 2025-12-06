use std::ops::{Add, Mul};

enum Operators {
    Plus,
    Times,
}

fn sum_vec(xs: Vec<i64>) -> i64 {
    xs.into_iter().reduce(Add::add).unwrap()
}

fn product_vec(xs: Vec<i64>) -> i64 {
    xs.into_iter().reduce(Mul::mul).unwrap()
}

impl Operators {
    fn from_string(op: &str) -> Self {
        match op {
            "*" => Self::Times,
            "+" => Self::Plus,
            _ => panic!("Invalid operator: {}", op),
        }
    }

    fn to_operator(self) -> fn(Vec<i64>) -> i64 {
        match self {
            Self::Plus => sum_vec,
            Self::Times => product_vec,
        }
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let lines: Vec<&str> = _input.lines().collect();
    let operator_line = lines.last().unwrap();
    let number_lines = &lines[..lines.len() - 1];
    let max_width =
        lines.iter().map(|l| l.len()).max().unwrap();

    let mut final_result = 0;
    let mut col_idx = 0;

    while col_idx < max_width {
        if is_column_whitespace(number_lines, col_idx) {
            col_idx += 1;
            continue;
        }

        let (problem_columns, problem_start_idx) =
            extract_problem_columns(
                number_lines,
                col_idx,
                max_width,
            );
        col_idx += problem_columns.len();

        let numbers =
            parse_numbers_from_columns(&problem_columns);

        let op_char = operator_line
            .chars()
            .nth(problem_start_idx)
            .unwrap_or('+');
        let op =
            Operators::from_string(&op_char.to_string())
                .to_operator();

        final_result += op(numbers);
    }

    Ok(final_result.to_string())
}

fn is_column_whitespace(
    number_lines: &[&str],
    col_idx: usize,
) -> bool {
    number_lines
        .iter()
        .map(|line| {
            line.chars().nth(col_idx).unwrap_or(' ')
        })
        .all(|c| c.is_whitespace())
}

fn extract_problem_columns(
    number_lines: &[&str],
    start_idx: usize,
    max_width: usize,
) -> (Vec<Vec<char>>, usize) {
    let mut columns = Vec::new();
    let mut col_idx = start_idx;

    while col_idx < max_width {
        let col_chars: Vec<char> = number_lines
            .iter()
            .map(|line| {
                line.chars().nth(col_idx).unwrap_or(' ')
            })
            .collect();

        if col_chars.iter().all(|c| c.is_whitespace()) {
            break;
        }

        columns.push(col_chars);
        col_idx += 1;
    }

    (columns, start_idx)
}

fn parse_numbers_from_columns(
    columns: &[Vec<char>],
) -> Vec<i64> {
    columns
        .iter()
        .rev()
        .filter_map(|column| {
            let num_str: String = column
                .iter()
                .filter(|c| !c.is_whitespace())
                .collect();

            if num_str.is_empty() {
                None
            } else {
                Some(num_str.parse::<i64>().unwrap())
            }
        })
        .collect()
}

fn celephod_col(col: Vec<i64>) -> Vec<i64> {
    let strings: Vec<String> =
        col.iter().map(|n| n.to_string()).collect();
    let max_len =
        strings.iter().map(|s| s.len()).max().unwrap();
    let mut result = Vec::new();

    for j in (0..max_len).rev() {
        let mut num = String::new();

        for s in &strings {
            if j < s.len() {
                num.push(s.chars().nth(j).unwrap());
            }
        }

        result.push(num.parse::<i64>().unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_celephod_col() {
        let input = vec![64, 23, 314];
        let expected_output = vec![4, 431, 623];
        let outcome = celephod_col(input);
        assert_eq!(outcome, expected_output);
    }

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
