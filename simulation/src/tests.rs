use super::*;

#[test]
fn test_3379() {
    for (rst, nums) in [
        (vec![1, 1, 1, 3], vec![3, -2, 1, 1]),
        (vec![-1, -1, 4], vec![-1, 4, -1]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol3379::construct_transformed_array(nums), rst);
        println!(":: {rst:?}");
    }
}
