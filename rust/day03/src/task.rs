use framework::Solution;
use std::convert::Infallible;
use std::str::FromStr;

pub struct Day03;
pub struct Joltages(Vec<Vec<u8>>);

impl FromStr for Joltages {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<char>> = s.lines()
                .map(|l| l.chars().collect::<Vec<char>>()).collect();

        let mut digits: Vec<Vec<u8>> = Vec::new();

        for mut line in lines {
            let _digits: Vec<u8> = line.iter_mut()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect();
            digits.push(_digits);
        }

        Ok(Joltages(digits))
    }
}

impl Solution for Day03 {
    type ParsedInput = Joltages;
    type ResultType = u64;

    fn part_one(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let mut sum = 0u64;

        for battery_line in input.0 {
            let biggest_digits = biggest_n_digits_in_order(battery_line, 2);
            sum += to_number(biggest_digits);
        }

        Some(sum)
    }

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let mut sum = 0u64;

        for battery_line in input.0 {
            let biggest_digits = biggest_n_digits_in_order(battery_line, 12);
            sum += to_number(biggest_digits);
        }

        Some(sum)
    }
}

pub(crate) fn biggest_n_digits_in_order(digits: Vec<u8>, n: u8) -> Vec<u8> {
    let mut res = vec![];
    let mut largest_idx = 0;

    for num in 1..=n {
        let mut curr_d = 0;
        for i in largest_idx..digits.len() - (n-num) as usize {
            if digits[i] > curr_d {
                curr_d = digits[i];
                largest_idx = i+1;
            }
        }
        res.push(curr_d);
    }

    res
}

pub(crate) fn to_number(digits: Vec<u8>) -> u64 {
    let len = digits.len();
    let mut num: u64 = 0;

    for i in 0..len {
        num += digits[i] as u64 * 10u64.pow((len - (i + 1)) as u32);
    }
    num
}