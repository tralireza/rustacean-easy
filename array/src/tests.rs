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

#[test]
fn test_2899() {
    for (rst, nums) in [
        (vec![2, 1, -1], vec![1, 2, -1, -1, -1]),
        (vec![1, 2, 1], vec![1, -1, 2, -1, -1]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2899::last_visited_integers(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2903() {
    for (rst, nums, index_difference, value_difference) in [
        (vec![0, 3], vec![5, 1, 4, 1], 2, 4),
        (vec![0, 0], vec![2, 1], 0, 0),
        (vec![-1, -1], vec![1, 2, 3], 2, 4),
    ] {
        println!("* {nums:?} {index_difference} {value_difference}");
        assert_eq!(
            Sol2903::find_indices(nums, index_difference, value_difference),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2908() {
    for (rst, nums) in [
        (9, vec![8, 6, 1, 5, 3]),
        (13, vec![5, 4, 8, 7, 10, 2]),
        (-1, vec![6, 5, 4, 3, 4, 5]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2908::minimum_sum(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2917() {
    for (rst, nums, k) in [
        (9, vec![7, 12, 9, 8, 9, 15], 4),
        (0, vec![2, 12, 1, 11, 4, 5], 6),
        (15, vec![10, 8, 5, 9, 11, 6, 8], 1),
    ] {
        println!("* {nums:?} {k}");
        assert_eq!(Sol2917::find_k_or(nums, k), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2946() {
    for (rst, mat, k) in [
        (false, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 4),
        (
            true,
            vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]],
            2,
        ),
        (true, vec![vec![2, 2], vec![2, 2]], 3),
    ] {
        println!("* {mat:?} {k}");
        assert_eq!(Sol2946::are_similar(mat, k), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2970() {
    for (rst, nums) in [
        (10, vec![1, 2, 3, 4]),
        (7, vec![6, 5, 7, 8]),
        (3, vec![8, 7, 6, 6]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2970::incremovable_subarray_count(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2974() {
    for (rst, nums) in [
        (vec![3, 2, 5, 4], vec![5, 4, 2, 3]),
        (vec![5, 2], vec![2, 5]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2974::number_game(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3028() {
    for (rst, nums) in [(1, vec![2, 3, -5]), (0, vec![3, 2, -3, -4])] {
        println!("* {nums:?}");
        assert_eq!(Sol3028::return_to_boundary_count(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3200() {
    for (rst, red, blue) in [(3, 2, 4), (2, 2, 1), (1, 1, 1), (2, 10, 1)] {
        println!("* {red} {blue}");
        assert_eq!(Sol3200::max_height_of_triangle(red, blue), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3127() {
    for (rst, grid) in [
        (
            true,
            vec![
                vec!['B', 'W', 'B'],
                vec!['B', 'W', 'W'],
                vec!['B', 'W', 'B'],
            ],
        ),
        (
            false,
            vec![
                vec!['B', 'W', 'B'],
                vec!['W', 'B', 'W'],
                vec!['B', 'W', 'B'],
            ],
        ),
    ] {
        println!("* {grid:?}");
        assert_eq!(Sol3127::can_make_square(grid), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3349() {
    for (rst, nums, k) in [
        (true, vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3),
        (false, vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 5),
        (false, vec![-15, 3, 16, 0], 2), // 1358/1422
    ] {
        println!("* {nums:?} {k}");
        assert_eq!(Sol3349::has_increasing_subarrays(nums, k), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3386() {
    for (rst, events) in [
        (1, vec![vec![1, 2], vec![2, 5], vec![3, 9], vec![1, 15]]),
        (10, vec![vec![10, 5], vec![1, 7]]),
        (
            2,
            vec![
                vec![9, 4],
                vec![19, 5],
                vec![2, 8],
                vec![3, 11],
                vec![2, 15],
            ],
        ), // 552/588
    ] {
        println!("* {events:?}");
        assert_eq!(Sol3386::button_with_longest_time(events), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3417() {
    for (rst, grid) in [
        (vec![1, 4], vec![vec![1, 2], vec![3, 4]]),
        (vec![2, 1, 2], vec![vec![2, 1], vec![2, 1], vec![2, 1]]),
        (
            vec![1, 3, 5, 7, 9],
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        ),
    ] {
        println!("* {grid:?}");
        assert_eq!(Sol3417::zigzag_traversal(grid), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3432() {
    for (rst, nums) in [
        (4, vec![10, 10, 3, 7, 6]),
        (0, vec![1, 2, 2]),
        (3, vec![2, 4, 6, 8]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol3432::count_partitions(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3452() {
    for (rst, nums, k) in [(12, vec![1, 3, 2, 1, 5, 4], 2), (2, vec![2, 1], 1)] {
        println!("* {nums:?} {k}");
        assert_eq!(Sol3452::sum_of_good_numbers(nums, k), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3502() {
    for (rst, cost) in [
        (vec![5, 3, 3, 1, 1, 1], vec![5, 3, 4, 1, 3, 2]),
        (vec![1, 1, 1, 1, 1], vec![1, 2, 4, 6, 7]),
    ] {
        println!("* {cost:?}");
        assert_eq!(Sol3502::min_costs(cost), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3550() {
    for (rst, nums) in [
        (2, vec![1, 3, 2]),
        (1, vec![1, 10, 11]),
        (-1, vec![1, 2, 3]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol3550::smallest_index(nums), rst);
        println!(":: {rst:?}");
    }
}
