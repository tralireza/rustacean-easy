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

#[test]
fn test_2614() {
    for (rst, nums) in [
        (11, vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]]),
        (17, vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 11, 10]]),
        (
            0,
            vec![
                vec![788, 645, 309, 559],
                vec![624, 160, 435, 724],
                vec![770, 483, 95, 695],
                vec![510, 779, 984, 238],
            ],
        ), // 72/82
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2614::diagonal_prime(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2639() {
    for (rst, grid) in [
        (vec![3], vec![vec![1], vec![22], vec![333]]),
        (
            vec![3, 1, 2],
            vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]],
        ),
        (vec![1], vec![vec![0]]),
    ] {
        println!("* {grid:?}");
        assert_eq!(Sol2639::find_column_width(grid), rst);
        println!(":: {rst:?}");
    }
}
