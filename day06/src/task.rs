use crate::task::Operation::{Add, Mul};
use framework::Solution;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Day06;

#[derive(Copy, Clone)]
enum Operation {
    Add,
    Mul,
}

struct FirstProblemList(Vec<Problem>);
struct SecondProblemList(Vec<Problem>);

struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl From<String> for FirstProblemList {
    fn from(value: String) -> Self {
        let mut problems: Vec<Problem> = vec![];

        let len = value.lines().count();
        let lines: Vec<&str> = value.lines().collect();
        let mut num_lines: Vec<Vec<u64>> = vec![];

        for i in 0..len - 1 {
            let num_line = lines[i]
                .split_whitespace()
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .collect::<Vec<u64>>();
            num_lines.push(num_line);
        }

        let op_line: Vec<Operation> = lines[len - 1]
            .split_whitespace()
            .map(|str| match str {
                "+" => Add,
                "*" => Mul,
                s => panic!("Invalid operation: {s}"),
            })
            .collect();

        let len = op_line.len();

        for i in 0..len {
            let mut nums: Vec<u64> = vec![];
            for num_line in &num_lines {
                nums.push(num_line[i]);
            }
            problems.push(Problem {
                operation: op_line[i].clone(),
                numbers: nums,
            });
        }

        FirstProblemList(problems)
    }
}

impl From<String> for SecondProblemList {
    fn from(value: String) -> Self {
        let mut problems: Vec<Problem> = vec![];

        let line_count = value.lines().count();
        let lines: Vec<&str> = value.lines().collect();

        let mut op_itr = lines[line_count - 1].chars();
        let mut ops: Vec<Operation> = vec![];

        let mut col_widths: Vec<usize> = vec![];
        let mut col_width = 0;

        // parse operators and column widths. assumes that
        // the operator row starts with an operator char
        while let Some(chr) = op_itr.next() {
            match chr {
                '+' => {
                    if col_width > 0 {
                        col_widths.push(col_width);
                        col_width = 0;
                    }
                    ops.push(Add);
                }
                '*' => {
                    if col_width > 0 {
                        col_widths.push(col_width);
                        col_width = 0;
                    }
                    ops.push(Mul);
                }
                ' ' => {
                    col_width += 1;
                }
                c => panic!("Invalid operation: {c}"),
            }
        }

        // catching the last column of the line, add 1 to width for the operator
        if col_width > 0 {
            col_width += 1;
            col_widths.push(col_width);
        }

        let mut num_cols: Vec<Vec<u64>> = vec![];
        let lines: Vec<Vec<char>> = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            //.inspect(|line| println!("line chars-len = {}", line.len()))
            .collect();
        let line_len = lines[0].len();
        let mut curr_col = 0;
        let mut widths_itr = col_widths.iter();

        while let Some(&curr_width) = widths_itr.next() {
            if curr_col > line_len {
                break;
            }

            let mut curr_num_col: Vec<u64> = vec![];

            // parse numbers column-wise
            for i in curr_col..(curr_col + curr_width) {
                let mut digits: Vec<u8> = vec![];

                // parse columns from top to bottom while skipping whitespace
                for line in &lines {
                    if line[i].is_numeric() {
                        digits.push(line[i].to_digit(10).unwrap() as u8);
                    }
                }
                let num = to_number(digits);
                curr_num_col.push(num);
            }

            num_cols.push(curr_num_col);

            curr_col += curr_width + 1;
        }

        for i in 0..ops.len() {
            problems.push(Problem {
                operation: ops[i].clone(),
                numbers: num_cols[i].clone(),
            });
        }

        SecondProblemList(problems)
    }
}

impl Debug for SecondProblemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for problem in &self.0 {
            write!(f, "{problem:?}")?;
        }
        Ok(())
    }
}

impl Debug for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Add => write!(f, "+")?,
            Mul => write!(f, "*")?,
        }
        Ok(())
    }
}

impl Debug for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        match self.operation {
            Add => write!(f, "+")?,
            Mul => write!(f, "*")?,
        }
        for num in &self.numbers {
            write!(f, " {num}")?;
        }
        write!(f, ")")?;

        Ok(())
    }
}

impl Problem {
    pub fn solve(&self) -> u64 {
        match &self.operation {
            Add => self.numbers.iter().sum(),
            Mul => self.numbers.iter().product(),
        }
    }
}

impl Solution for Day06 {
    type ParsedInput = String;
    type ResultType = u64;

    fn part_one(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let parsed: FirstProblemList = input.into();
        let result = parsed.0.iter().map(Problem::solve).sum();

        Some(result)
    }

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let parsed: SecondProblemList = input.into();
        let result = parsed.0.iter().map(Problem::solve).sum();

        Some(result)
    }
}

pub(crate) fn to_number(digits: Vec<u8>) -> u64 {
    let len = digits.len();
    let mut num: u64 = 0;

    for i in 0..len {
        num += digits[i] as u64 * 10u64.pow((len - (i + 1)) as u32);
    }
    num
}
