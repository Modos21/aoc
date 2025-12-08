pub mod task;

#[cfg(test)]
mod tests {
    use crate::task::{rotate, Day01};
    use framework::{testfile, Part, Solution};

    macro_rules! rot_test {
        ($num:literal, $rot:literal, $exp_num:literal, $exp_0s:literal) => {
            let (pos, over_0) = rotate($num, $rot);
            assert_eq!(pos, $exp_num);
            assert_eq!(over_0, $exp_0s);
        };
    }

    #[test]
    fn test_password_1() {
        if let Some(pw) = Day01::run(Part::Two, testfile!("test1")) {
            assert_eq!(pw, 2);
        }
    }

    #[test]
    fn test_password_2() {
        if let Some(pw) = Day01::run(Part::Two, testfile!("test2")) {
            assert_eq!(pw, 3);
        }
    }

    #[test]
    fn test_password_3() {
        if let Some(pw) = Day01::run(Part::Two, testfile!("test3")) {
            assert_eq!(pw, 6);
        }
    }

    #[test]
    fn test_password_4() {
        if let Some(pw) = Day01::run(Part::Two, testfile!("test4")) {
            assert_eq!(pw, 10);
        }
    }

    #[test]
    fn test_rotates_over_0() {
        rot_test!(50, -1000, 50, 10);
    }

    #[test]
    fn test_rotate_1_n1() {
        rot_test!(1, -2, 99, 1);
    }

    #[test]
    fn test_rotate_77_523() {
        rot_test!(77, 523, 0, 6);
    }

    #[test]
    fn test_rotate_0_1() {
        rot_test!(0, 1, 1, 0);
    }

    #[test]
    fn test_rotate_50_150() {
        rot_test!(50, 150, 0, 2);
    }

    #[test]
    fn test_rotate_50_550() {
        rot_test!(50, 550, 0, 6);
    }
}