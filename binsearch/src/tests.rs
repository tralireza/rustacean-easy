use super::*;

#[test]
fn test_1099() {
    for (rst, nums, k) in [
        (58, vec![34, 23, 1, 24, 75, 33, 54, 8], 60),
        (-1, vec![10, 20, 30], 16),
    ] {
        println!("* {nums:?} {k}");
        assert_eq!(Sol1099::two_sum_less_than_k(nums, k), rst);
        println!(":: {rst:?}");
    }
}
