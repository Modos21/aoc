use framework::Solution;
use std::convert::Infallible;
use std::str::FromStr;

pub struct Day02;

pub struct Ranges(Vec<(u64, u64)>);

impl FromStr for Ranges {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s.split(",");
        let mut range_tuples: Vec<(u64, u64)> = Vec::new();

        for range in ranges {
            let (start, end) = range.split_once("-").unwrap();
            let (start, end) = (start.parse::<u64>().expect("failed to parse to u64"),
                                end.parse::<u64>().expect("failed to parse to u64"));
            range_tuples.push((start, end));
        }

        Ok(Ranges(range_tuples))
    }
}

impl Solution for Day02 {
    type ParsedInput = Ranges;
    type ResultType = u64;

    fn part_one(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let invalid_ids = find_invalid_ids(input.0, is_valid_id_1);
        let sum = invalid_ids.iter().sum::<u64>();
        Some(sum)
    }

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let invalid_ids = find_invalid_ids(input.0, is_valid_id_2);
        let sum = invalid_ids.iter().sum::<u64>();
        Some(sum)
    }
}

fn find_invalid_ids<F>(id_ranges: Vec<(u64, u64)>, is_valid_func: F) -> Vec<u64>
where F: Fn(u64) -> bool {
    let mut invalid_ids: Vec<u64> = Vec::new();

    id_ranges.iter().for_each(|tuple| {
        //println!("current range: ({} to {})", tuple.0, tuple.1);
        for id in tuple.0..=tuple.1 {
            if !is_valid_func(id) {
                invalid_ids.push(id);
            }
        }
    });

    invalid_ids
}

pub(crate) fn is_valid_id_1(id: u64) -> bool {
    let id_str: String = id.to_string();
    if id_str.len() % 2 == 1 { return true }
    let (left, right) = id_str.split_at(id_str.len() / 2);
    left != right
}

pub(crate) fn is_valid_id_2(id: u64) -> bool {
    let id_str: String = id.to_string();
    let len = id_str.len();
    if len == 1 { return true }
    let max_test_len = len / 2;

    for n in 1..=max_test_len {
        let chars = id_str.chars().collect::<Vec<char>>();
        let mut chunks = chars.chunks(n);
        let pattern = chunks.next().unwrap();
        if chunks.all(|ch| ch == pattern) {
            return false
        }
    }
    true
}