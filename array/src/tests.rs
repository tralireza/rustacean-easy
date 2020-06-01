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
