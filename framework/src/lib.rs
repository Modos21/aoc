use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

pub enum Part {
    One,
    Two,
}

pub trait Solution {
    type ParsedInput: FromStr<Err = Infallible>;
    type ResultType: Display + Clone;

    fn run(part: Part, input: &str) -> Option<Self::ResultType> {
        let parsed = Self::ParsedInput::from_str(input).expect("Parsing should never fail");

        let (part, res) = match part {
            Part::One =>  ("1", Self::part_one(parsed)),
            Part::Two => ("2", Self::part_two(parsed)),
        };

        match res.clone() {
            None => println!("part {part} not yet implemented!"),
            Some(v) => println!("Part {part} result: {}", v),
        }

        res
    }

    fn part_one(input: Self::ParsedInput) -> Option<Self::ResultType>;

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType>;
}

#[macro_export]
macro_rules! run {
    ( $day:ty | $part:expr $(, $part2:expr)? ) => {
        const INPUT: &str = include_str!("../input/input.txt");
        <$day>::run($part, INPUT);
        $(<$day>::run($part2, INPUT);)?
    };

    ( $day:ty, part1 ) => {
        $crate::run!($day | Part::One);
    };

    ( $day:ty, part1, part2 ) => {
        $crate::run!($day | Part::One, Part::Two);
    };

    ( $day:ty, part2 ) => {
        $crate::run!($day | Part::Two);
    };
}

#[macro_export]
macro_rules! testfile {
    ($name:literal) => { include_str!(concat!("../test/", $name, ".in")) };
}

#[macro_export]
macro_rules! set {
    // full version with mapping and filtering
    ( $var:ident in $src:expr; $e:expr; where $cond:expr ) => {{
        $src.into_iter()
            .filter(|$var| $cond)
            .map(|$var| $e)
            .collect::<Vec<_>>()
    }};

    // no filtering but mapping
    ( $var:ident in $src:expr; $e:expr ) => {{ $src.into_iter().map(|$var| $e).collect::<Vec<_>>() }};

    // no mapping but filtering
    ( $var:ident in $src:expr; where $cond:expr ) => {{ $src.into_iter().filter(|$var| $cond).collect::<Vec<_>>() }};

    // all elements from src
    ( $var:ident in $src:expr ) => {{ $src.into_iter().collect::<Vec<_>>() }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        assert_eq!(vec![1, 2, 3, 4, 5], set! { x in 1..6 });
    }

    #[test]
    fn test_set_map_filter() {
        assert_eq!(
            vec![1, 9, 25, 49, 81],
            set! { x in 1..=10; x * x; where x % 2 != 0 }
        );
    }

    #[test]
    fn test_set_filter() {
        assert_eq!(vec![1, 3, 5, 7, 9], set! { x in 1..=10; where x % 2 != 0 });
    }

    #[test]
    fn test_set_local_var() {
        let v = vec![1, 2, 3];
        assert_eq!(vec![1, 2, 3], set! { x in v });
    }
}
