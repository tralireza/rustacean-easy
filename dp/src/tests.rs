use super::*;

#[test]
fn test_198() {
    for (rst, nums) in [(4, vec![1, 2, 3, 1]), (12, vec![2, 7, 9, 3, 1])] {
        println!("* {nums:?}");
        assert_eq!(Sol198::rob(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_740() {
    for (rst, nums) in [
        (6, vec![3, 4, 2]),
        (9, vec![2, 2, 3, 3, 3, 4]),
        (21, vec![5, 4, 5, 4, 3, 5, 3]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol740::delete_and_earn(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_746() {
    for (rst, cost) in [
        (15, vec![10, 15, 20]),
        (6, vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
    ] {
        println!("* {cost:?}");
        assert_eq!(Sol746::min_cost_climbing_stairs(cost), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1137() {
    for (rst, n) in [(4, 4), (1389537, 25)] {
        println!("* {n}");
        assert_eq!(Sol1137::tribonacci(n), rst);
        println!(":: {rst:?}");
    }
}
