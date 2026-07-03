mod task;

#[cfg(test)]
mod tests {
    use crate::task::{Day05, ProductIdRanges};
    use framework::{testfile, Solution};
    use std::str::FromStr;

    #[test]
    fn test_file1() {
        let input = ProductIdRanges::from_str(testfile!("test1")).unwrap();
        if let Some(res) = Day05::part_two(input) {
            print!("Number of unique IDs in all ranges: {res}");
            assert_eq!(res, 14);
        }
    }

    #[test]
    fn test_file2() {
        let input = ProductIdRanges::from_str(testfile!("test2")).unwrap();
        if let Some(res) = Day05::part_two(input) {
            print!("Number of unique IDs in all ranges: {res}");
            assert_eq!(res, 100_033);
        }
    }

    #[test]
    fn test_file3() {
        let input = ProductIdRanges::from_str(testfile!("test3")).unwrap();
        if let Some(res) = Day05::part_two(input) {
            print!("Number of unique IDs in all ranges: {res}");
            assert_eq!(res, 226);
        }
    }

    #[test]
    fn test_file4() {
        let input = ProductIdRanges::from_str(testfile!("test4")).unwrap();
        if let Some(res) = Day05::part_two(input) {
            print!("Number of unique IDs in all ranges: {res}");
            assert_eq!(res, 4);
        }
    }
}