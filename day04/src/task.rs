use framework;
use framework::{do_while, Solution};
use std::convert::Infallible;
use std::str::FromStr;

pub struct Day04;
pub struct PaperRolls(Vec<Vec<bool>>);

impl FromStr for PaperRolls {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<bool>> = Vec::new();

        for line in s.lines() {
            let rolls: Vec<bool> = line.chars()
                .map(|ch| match ch {
                    '.' => false,
                    '@' => true,
                    c => panic!("Invalid char: {c}"),
                }).collect();

            rows.push(rolls);
        }

        Ok(PaperRolls(rows))
    }
}

impl Solution for Day04 {
    type ParsedInput = PaperRolls;
    type ResultType = u64;

    fn part_one(input: Self::ParsedInput) -> Option<u64> {
        let max_row = (input.0.len() - 1) as u64;
        let max_col = (input.0[0].len() - 1) as u64;

        let mut removable_rolls: u64 = 0;

        for row in 0..=max_row {
            for col in 0..=max_col {
                if !input.0[row as usize][col as usize] {
                    //print!(" ");
                    continue
                }

                let neighbours = num_neighbours(&input.0, (row, col), (max_row, max_col));
                //print!("{}", neighbours);
                if neighbours < 4 {
                    removable_rolls += 1;
                }
            }
            //println!();
        }

        Some(removable_rolls)
    }

    fn part_two(mut input: Self::ParsedInput) -> Option<u64> {
        let max_row = (input.0.len() - 1) as u64;
        let max_col = (input.0[0].len() - 1) as u64;

        let mut removed_rolls: u64 = 0;
        let mut removed;
        do_while!{{
            removed = 0;

            for row in 0..=max_row {
                for col in 0..=max_col {
                    if !input.0[row as usize][col as usize] {
                        //print!(" ");
                        continue
                    }

                    let neighbours = num_neighbours(&input.0, (row, col), (max_row, max_col));
                    //print!("{}", neighbours);
                    if neighbours < 4 {
                        input.0[row as usize][col as usize] = false;
                        removed_rolls += 1;
                        removed += 1;
                    }
                }
                //println!();
            }
        } while removed > 0 }

        Some(removed_rolls)
    }
}

pub(crate) fn num_neighbours(matrix: &Vec<Vec<bool>>, pos: (u64, u64), dims: (u64, u64)) -> u8 {
    let max_row = dims.0;
    let max_col = dims.1;
    let from_row = if pos.0 > 1 { pos.0 - 1 } else { 0 };
    let to_row = if pos.0 < max_row { pos.0 + 1 } else { max_row };
    let from_col = if pos.1 > 1 { pos.1 - 1 } else { 0 };
    let to_col = if pos.1 < max_col { pos.1 + 1 } else { max_col };

    let mut neighbours: u8 = 0;

    for row in from_row..=to_row {
        for col in from_col..=to_col {
            if (row, col) == (pos.0, pos.1) {
                continue
            }

            if matrix[row as usize][col as usize] {
                neighbours += 1;
            }
        }
    }

    neighbours
}
