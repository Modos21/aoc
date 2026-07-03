mod task;

#[cfg(test)]
mod tests {
    use crate::task::{create_circuits, Vec3U, VecList};
    use framework::testfile;
    use std::str::FromStr;

    #[test]
    fn test() {
        let vec_list: VecList = VecList::from_str("123, 123, 123").unwrap();
        let vec: &Vec3U = &vec_list.0[0];

        assert_eq!(
            vec,
            &Vec3U {
                x: 123,
                y: 123,
                z: 123
            }
        );
    }

    #[test]
    fn test_file_1() {
        let parsed = VecList::from_str(testfile!("test1")).unwrap();

        let circ_sizes = create_circuits(&parsed, 10);

        dbg!(circ_sizes);
    }
}
