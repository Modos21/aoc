mod task;

#[cfg(test)]
mod tests {
    use crate::task::{Day05, ProductIdRanges};
    use framework::{testfile, Solution};
    use std::str::FromStr;

    #[test]
    fn test_() {
        let input = ProductIdRanges::from_str(testfile!("test1")).unwrap();
        if let Some(res) = Day05::part_two(input) {
            print!("Number of unique IDs in all ranges: {res}");
            assert_eq!(res, 14);
        }
    }
}