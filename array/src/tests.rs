use super::*;

#[test]
fn test_118() {
    for (rst, row_nums) in [
        (
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ],
            5,
        ),
        (vec![vec![1]], 1),
    ] {
        println!("* {row_nums}");
        assert_eq!(Sol118::generate(row_nums), rst);
        println!(":: {rst:?}");
    }
}

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

#[test]
fn test_2660() {
    for (rst, player1, player2) in [
        (1, vec![5, 10, 3, 2], vec![6, 5, 7, 3]),
        (2, vec![3, 5, 7, 6], vec![8, 10, 10, 2]),
        (0, vec![2, 3], vec![4, 1]),
        (
            2,
            vec![1, 1, 1, 10, 10, 10, 10],
            vec![10, 10, 10, 10, 1, 1, 1],
        ),
    ] {
        println!("* {player1:?} {player2:?}");
        assert_eq!(Sol2660::is_winner(player1, player2), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2760() {
    for (rst, nums, threshold) in [
        (3, vec![3, 2, 5, 4], 5),
        (1, vec![1, 2], 2),
        (3, vec![2, 3, 4, 5], 4),
        (0, vec![1, 6], 2), // 6710 / 6873
    ] {
        println!("* {nums:?} {threshold}");
        assert_eq!(Sol2760::longest_alternating_subarray(nums, threshold), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2765() {
    for (rst, nums) in [
        (4, vec![2, 3, 4, 3, 4]),
        (2, vec![4, 5, 6]),
        (4, vec![31, 32, 31, 32, 33]), // 1532/2880
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2765::alternating_subarray(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2848() {
    for (rst, nums) in [
        (7, vec![vec![3, 6], vec![1, 5], vec![4, 7]]),
        (7, vec![vec![1, 3], vec![5, 8]]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2848::number_of_points(nums), rst);
        println!(":: {rst:?}");
    }
}
