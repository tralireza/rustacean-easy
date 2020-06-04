use super::*;

#[test]
fn test_2389() {
    for (rst, nums, queries) in [
        (vec![2, 3, 4], vec![4, 5, 2, 1], vec![3, 10, 21]),
        (vec![0], vec![2, 3, 4, 5], vec![1]),
    ] {
        println!("* {nums:?} {queries:?}");
        assert_eq!(Sol2389::answer_queries(nums, queries), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2404() {
    for (rst, nums) in [
        (2, vec![0, 1, 2, 2, 4, 4, 1]),
        (4, vec![4, 4, 4, 9, 2, 4]),
        (-1, vec![29, 47, 21, 41, 13, 37, 25, 7]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2404::most_frequent_even(nums), rst);
        println!(":: {rst:?}");
    }
}
