pub mod task;

#[cfg(test)]
mod tests {
    use crate::task::{is_valid_id_1, is_valid_id_2};

    #[test]
    fn test_invalid_ids_part1() {
        assert!(!is_valid_id_1(123_123));
        assert!(!is_valid_id_1(42_42));
        assert!(!is_valid_id_1(1_1));
        assert!(!is_valid_id_1(434_434));
    }

    #[test]
    fn test_valid_ids_part1() {
        assert!(is_valid_id_1(123));
        assert!(is_valid_id_1(424));
        assert!(is_valid_id_1(101));
    }

    #[test]
    fn test_invalid_ids_part2() {
        assert!(!is_valid_id_2(123_123_123));
        assert!(!is_valid_id_2(42_42));
        assert!(!is_valid_id_2(1_1_1));
        assert!(!is_valid_id_2(434_434));
    }

    #[test]
    fn test_range_11_22() {
        for i in 11..=22 {
            println!("{} valid = {}", i, is_valid_id_1(i));
        }
    }

    #[test]
    fn test_file1() {
        //run("test/test1.in").unwrap();
    }

    #[test]
    fn test_run() {
        //run("input/input.txt").unwrap();
    }
}