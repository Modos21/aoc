mod task;

#[cfg(test)]
mod tests {
    use crate::task::{Day04, PaperRolls};
    use framework::Solution;
    use std::str::FromStr;

    #[test]
    fn test_3x3_matrix() {
        let input = ".@.\n@@@\n.@.";

        if let Some(ans) = Day04::part_one(PaperRolls::from_str(input).unwrap()) {
            assert_eq!(ans, 4);
        }
    }
}