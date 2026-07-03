mod task;

#[cfg(test)]
mod tests {
    use crate::task::{Day07, ManifoldDiagram};
    use framework::{testfile, Solution};
    use std::str::FromStr;

    #[test]
    fn test_file1_part1() {
        let parsed = ManifoldDiagram::from_str(testfile!("test1")).unwrap();
        if let Some(splits) = Day07::part_one(parsed) {
            assert_eq!(splits, 21);
        }
    }


    #[test]
    fn test_file1_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test1")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 40);
        }
    }

    #[test]
    fn test_file2_part1() {
        let parsed = ManifoldDiagram::from_str(testfile!("test2")).unwrap();
        if let Some(splits) = Day07::part_one(parsed) {
            assert_eq!(splits, 2);
        }
    }


    #[test]
    fn test_file2_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test2")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 3);
        }
    }

    #[test]
    fn test_file3_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test3")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 2);
        }
    }

    #[test]
    fn test_file4_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test4")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 1);
        }
    }

    #[test]
    fn test_file5_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test5")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 4);
        }
    }

    #[test]
    fn test_file6_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test6")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 1);
        }
    }

    #[test]
    fn test_file7_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test7")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 4);
        }
    }

    #[test]
    fn test_file8_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test8")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 7);
        }
    }

    #[test]
    fn test_file9_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test9")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 6);
        }
    }

    #[test]
    fn test_file10_part2() {
        let parsed = ManifoldDiagram::from_str(testfile!("test10")).unwrap();
        if let Some(paths) = Day07::part_two(parsed) {
            assert_eq!(paths, 64);
        }
    }
}