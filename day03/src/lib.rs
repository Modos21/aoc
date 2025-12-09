pub mod task;

#[cfg(test)]
mod tests {
    use crate::task::{to_number, Day03};
    use framework::{testfile, Part, Solution};

    #[test]
    fn test_file1() {
        if let Some(sum) = Day03::run(Part::One, testfile!("test1")) {
            assert_eq!(sum, 89);
        }
    }

    #[test]
    fn test_file2() {
        if let Some(sum) = Day03::run(Part::One, testfile!("test2")) {
            assert_eq!(sum, 89);
        }
    }

    #[test]
    fn test_to_number() {
        assert_eq!(54321, to_number(vec![5, 4, 3, 2, 1]));
    }
}