use super::*;

#[test]
fn test_3667() {
    for (rst, nums) in [
        (vec![-1, 1, 3, -4, 5], vec![3, -1, -4, 1, 5]),
        (vec![-100, 100], vec![-100, 100]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol3667::sort_by_absolute_value(nums), rst);
        println!(":: {rst:?}");
    }
}
