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

#[test]
fn test_1150() {
    for (rst, nums, target) in [
        (true, vec![2, 4, 5, 5, 5, 5, 5, 6, 6], 5),
        (false, vec![10, 100, 101, 101], 101),
    ] {
        println!("* {nums:?} {target}");
        assert_eq!(Sol1150::is_majority_element(nums, target), rst);
        println!(":: {rst:?}");
    }
}
