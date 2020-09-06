use super::*;

#[test]
fn test_198() {
    for (rst, nums) in [
        (4, vec![1,2,3,1]),
        (12, vec![2,7,9,3,1]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol198::rob(nums), rst);
        println!(":: {rst:?}");
    }
}
