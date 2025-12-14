mod task;

#[cfg(test)]
mod tests {
    use crate::task::{Day07, ManifoldDiagram};
    use framework::{testfile, Solution};
    use std::str::FromStr;

    #[test]
    fn test_file1() {
        let parsed = ManifoldDiagram::from_str(testfile!("test1")).unwrap();
        if let Some(splits) = Day07::part_one(parsed) {
            assert_eq!(splits, 21);
        }
    }

    #[test]
    fn test_file2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test2")).unwrap();
        if let Some(splits) = Day07::part_one(parsed) {
            assert_eq!(splits, 2);
        }
    }
}