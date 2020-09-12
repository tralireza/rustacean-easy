use super::*;

#[test]
fn test_1493() {
    for (rst, nums) in [
        (3, vec![1, 1, 0, 1]),
        (5, vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
        (2, vec![1, 1, 1]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol1493::longest_subarray(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3411() {
    for (rst, nums) in [
        (5, vec![1, 2, 1, 2, 1, 1, 1]),
        (3, vec![2, 3, 4, 5, 6]),
        (5, vec![1, 2, 3, 1, 4, 5, 1]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol3411::max_length(nums), rst);
        println!(":: {rst:?}");
    }
}
