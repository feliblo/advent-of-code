use std::{
    ops::{Add, Mul},
    thread::current,
};
use tracing::info;
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
            _ => panic!("Does another operator exist?"),
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
    let mut rows: Vec<Vec<&str>> = _input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let operators: Vec<fn(Vec<i64>) -> i64> = rows
        .pop()
        .unwrap()
        .into_iter()
        .map(|a| Operators::from_string(a).to_operator())
        .collect();

    let column_len = rows[0].len();
    let mut iterator: Vec<_> =
        rows.into_iter().map(|n| n.into_iter()).collect();
    let columns = (0..column_len)
        .map(|_| {
            iterator
                .iter_mut()
                .map(|n| {
                    let current_n: i64 = n
                        .next()
                        .unwrap()
                        .parse()
                        .expect("Should be parsable");
                    println!("{}", current_n);
                    current_n
                })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut final_result = 0;
    for (i, column) in columns.iter().enumerate() {
        let op = operators[i];
        let result = op(column.clone());
        final_result += result
    }
    Ok(final_result.to_string())
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
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
