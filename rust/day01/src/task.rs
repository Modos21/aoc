use framework::Solution;
use std::str::FromStr;
use std::string::ParseError;

pub struct Day01;

#[derive(Debug)]
pub struct RotationList(Vec<i32>);

impl FromStr for RotationList {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rotations = RotationList(vec![]);

        for line in s.lines() {
            let rot = match &line[0..1] {
                "L" => -1,
                "R" => 1,
                c => panic!("invalid rotation char: {}", c)
            };

            let num: i32 = match &line[1..].trim().parse::<i32>() {
                Ok(n) => n,
                Err(_) => panic!("couldn't parse to i32"),
            }.to_owned();

            rotations.0.push(num * rot);
        }

        Ok(rotations)
    }
}

impl Solution for Day01 {
    type ParsedInput = RotationList;
    type ResultType = u64;

    fn part_one(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let mut dial_pos: u64 = 50;
        let mut num_zeros: u64 = 0;

        for rot in input.0 {
            (dial_pos, _) = rotate(dial_pos, rot);
            if dial_pos == 0 {
                num_zeros += 1;
            }
        }

        Some(num_zeros)
    }

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let mut dial_pos: u64 = 50;
        let mut num_zeros: u64 = 0;
        let mut went_over_0: u64;

        for rot in input.0 {
            (dial_pos, went_over_0) = rotate(dial_pos, rot);
            num_zeros += went_over_0;
        }

        Some(num_zeros)
    }
}


pub(crate) fn rotate(num: u64, rotation: i32) -> (u64, u64) {
    let new_num = num as i32 + rotation;
    let mut rotations = (new_num / 100).abs();
    let new_dial = new_num.rem_euclid(100);

    if num != 0 && new_num <= 0 {
        rotations += 1
    }

    (new_dial as u64, rotations as u64)
}
