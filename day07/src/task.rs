use crate::task::DiagramElement::{Beam, Empty, Source, Splitter};
use framework::Solution;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct Day07;

#[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum DiagramElement {
    Source,
    Splitter,
    Beam,
    Empty
}

impl Debug for DiagramElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Source => write!(f, "S")?,
            Beam => write!(f, "|")?,
            Splitter => write!(f, "^")?,
            Empty => write!(f, ".")?,
        }

        Ok(())
    }
}

pub type Diagram = Vec<Vec<DiagramElement>>;

pub struct ManifoldDiagram(Diagram);

impl FromStr for ManifoldDiagram {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<DiagramElement>> = vec![];

        for line in s.lines() {
            let mut row: Vec<DiagramElement> = vec![];
            for char in line.chars() {
                let elem: DiagramElement = match char {
                    'S' => Source,
                    '^' => Splitter,
                    '.' => Empty,
                    c => panic!("Invalid char: {c}")
                };
                row.push(elem);
            }
            rows.push(row);
        }

        Ok(ManifoldDiagram(rows))
    }
}

impl Debug for ManifoldDiagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for elem in line {
                write!(f, "{elem:?}")?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Solution for Day07 {
    type ParsedInput = ManifoldDiagram;
    type ResultType = u64;

    fn part_one(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let mut diagram = input.0;

        let source_pos = diagram[0].binary_search(&Source).unwrap();
        let row_len = diagram.len();
        let col_len = diagram[0].len();
        let curr_pos: (usize, usize) = (0, source_pos);

        let splits = calc_splits(&mut diagram, curr_pos, (row_len, col_len));

        print_diagram(&diagram);

        Some(splits)
    }

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let diagram = input.0;
        let source_pos = diagram[0].binary_search(&Source).unwrap();
        let row_len = diagram.len();
        let col_len = diagram[0].len();
        let curr_pos: (usize, usize) = (0, source_pos);

        let paths = calc_paths_iter(&diagram, curr_pos, (row_len, col_len));

        println!("#paths = {paths}");

        Some(paths)
    }
}

pub(crate) fn calc_splits(diagram: &mut Diagram, mut pos: (usize, usize), lens: (usize, usize))
-> u64 {
    let mut splits = 0;

    while pos.0 < lens.0 {
        match &diagram[pos.0][pos.1] {
            Splitter => {
                splits += 1;
                // if there is space on the left, check for splits in that adjacent column
                if pos.1 > 0 {
                    splits += calc_splits(diagram, (pos.0, pos.1 - 1), lens);
                }
                // if there is space on the right, check for splits in that adjacent column
                if pos.1 < lens.1 {
                    splits += calc_splits(diagram, (pos.0, pos.1 + 1), lens);
                }
                break;
            },
            Beam => {
                break;
            },
            Empty => {
                diagram[pos.0][pos.1] = Beam;
                pos.0 += 1;
            },
            Source => {
                pos.0 += 1;
            }
        }
    }

    splits
}

/// Calculates the number of paths you can take from the source under the assumption that at
/// every splitter you can choose to go left or right
///
/// Caution: this method works recursively and is insanely slow for larger inputs
/// Time complexity: O(2^N) where N is the number of lines in the input
pub(crate) fn calc_paths(diagram: &Diagram, mut pos: (usize, usize), lens: (usize, usize)) -> u64 {
    while pos.0 < lens.0 {
        match &diagram[pos.0][pos.1] {
            Splitter => {
                let mut l = 0;
                let mut r = 0;

                // if there is space on the left, check for splits in that adjacent column
                if pos.1 > 0 {
                    l = calc_paths(&diagram, (pos.0, pos.1 - 1), lens);
                }
                // if there is space on the right, check for splits in that adjacent column
                if pos.1 < lens.1 {
                    r = calc_paths(&diagram, (pos.0, pos.1 + 1), lens);
                }

                return l + r
            },
            Empty | Source => {
                pos.0 += 1;
            },
            _ => {}
        }
    }

    1
}

pub(crate) fn calc_paths_iter(diagram: &Diagram, start: (usize, usize), lens: (usize, usize)) -> u64 {
    let rows = lens.0;
    let cols = lens.1;
    if start.0 >= rows || start.1 >= cols {
        return 0;
    }

    let mut memo: Vec<Vec<Option<u64>>> = vec![vec![None; cols]; rows];
    // stack: (row, col, processed_flag)
    let mut stack: Vec<(usize, usize, bool)> = Vec::new();
    stack.push((start.0, start.1, false));

    while let Some((r, c, processed)) = stack.pop() {
        if r >= rows || c >= cols {
            continue;
        }

        // already computed
        if memo[r][c].is_some() {
            continue;
        }

        if processed {
            // children must be computed now
            let val = match diagram[r][c] {
                Splitter => {
                    let mut sum: u64 = 0;
                    if c > 0 {
                        if let Some(v) = memo[r][c - 1] {
                            sum = sum.saturating_add(v);
                        }
                    }
                    if c + 1 < cols {
                        if let Some(v) = memo[r][c + 1] {
                            sum = sum.saturating_add(v);
                        }
                    }
                    sum
                }
                Empty | Source => {
                    if r + 1 >= rows {
                        1
                    } else {
                        memo[r + 1][c].unwrap_or(0)
                    }
                }
                _ => 0,
            };
            memo[r][c] = Some(val);
        } else {
            // first time visit: either resolve immediate base cases or push children then self (processed)
            match diagram[r][c] {
                Empty | Source => {
                    if r + 1 >= rows {
                        memo[r][c] = Some(1);
                    } else {
                        // ensure child computed first
                        stack.push((r, c, true));
                        if memo[r + 1][c].is_none() {
                            stack.push((r + 1, c, false));
                        }
                    }
                }
                Splitter => {
                    // push self (processed) after children
                    stack.push((r, c, true));
                    if c + 1 < cols && memo[r][c + 1].is_none() {
                        stack.push((r, c + 1, false));
                    }
                    if c > 0 && memo[r][c - 1].is_none() {
                        stack.push((r, c - 1, false));
                    }
                }
                _ => {
                    memo[r][c] = Some(0);
                }
            }
        }
    }

    memo[start.0][start.1].unwrap_or(0)
}

fn print_diagram(d: &Diagram) {
    for line in d {
        for elem in line {
            match elem {
                Source => print!("S"),
                Beam => print!("|"),
                Splitter => print!("^"),
                Empty => print!("."),
            }
        }
        println!();
    }
}