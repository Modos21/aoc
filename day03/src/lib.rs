pub mod task;

#[cfg(test)]
mod tests {
    use crate::task::{run, to_number, Part};

    #[test]
    fn test_file1() {
        let sum = run("test/test1.in", Part::One).unwrap();
        assert_eq!(sum, 89);
    }

    #[test]
    fn test_file2() {
        let sum = run("test/test2.in", Part::One).unwrap();
        assert_eq!(sum, 357);
    }

    #[test]
    fn test_file2_p2() {
        let _ = run("test/test2.in", Part::Two).unwrap();
    }

    #[test]
    fn test_to_number() {
        assert_eq!(54321, to_number(vec![5, 4, 3, 2, 1]));
    }
}