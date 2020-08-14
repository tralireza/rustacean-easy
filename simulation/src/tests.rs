use super::*;

#[test]
fn test_3354() {
    for (rst, nums) in [
        (2, vec![1, 0, 2, 0, 3]),
        (0, vec![2, 3, 4, 0, 4, 1, 0]),
        (3, vec![16, 13, 10, 0, 0, 0, 10, 6, 7, 8, 7]), // 285/584
        (2, vec![0]),                                   // 519/584
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol3354::count_valid_selections(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3360() {
    for (rst, n) in [(true, 12), (false, 1)] {
        println!("* {n}");
        assert_eq!(Sol3360::can_alice_win(n), rst);
        println!(":: {rst:?}");
    }
}

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
