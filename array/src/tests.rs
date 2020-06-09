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

#[test]
fn test_2432() {
    for (rst, n, logs) in [
        (1, 10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]),
        (
            3,
            26,
            vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]],
        ),
        (0, 2, vec![vec![0, 10], vec![1, 20]]),
    ] {
        println!("* {n} {logs:?}");
        assert_eq!(Sol2432::hardest_worker(n, logs), rst);
        println!(":: {rst:?}");
    }
}
