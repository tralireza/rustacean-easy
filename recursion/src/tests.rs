use super::*;

#[test]
fn test_37() {
    for (rst, board) in [(
        [
            ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
            ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
            ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
            ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
            ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
            ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
            ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
            ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
            ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
        ],
        [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ],
    )] {
        let mut board: Vec<Vec<_>> = board
            .into_iter()
            .map(|row| row.into_iter().flat_map(|s| s.chars().nth(0)).collect())
            .collect();
        let rst: Vec<Vec<_>> = rst
            .into_iter()
            .map(|row| row.into_iter().flat_map(|s| s.chars().nth(0)).collect())
            .collect();

        for row in &board {
            println!("* {row:?}");
        }

        Sol37::solve_sudoku(&mut board);
        assert_eq!(board, rst);

        for row in &rst {
            println!(":: {row:?}");
        }
    }
}

#[test]
fn test_1066() {
    for (rst, workers, bikes) in [
        (6, vec![[0, 0], [2, 1]], vec![[1, 2], [3, 3]]),
        (
            4,
            vec![[0, 0], [1, 1], [2, 0]],
            vec![[1, 0], [2, 2], [2, 1]],
        ),
        (
            4995,
            vec![[0, 0], [1, 0], [2, 0], [3, 0], [4, 0]],
            vec![[0, 999], [1, 999], [2, 999], [3, 999], [4, 999]],
        ),
    ] {
        println!("* {workers:?} {bikes:?}");
        assert_eq!(
            Sol1066::assign_bikes(
                workers
                    .into_iter()
                    .map(|a| a.into_iter().collect())
                    .collect(),
                bikes.into_iter().map(|a| a.into_iter().collect()).collect(),
            ),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3483() {
    for (rst, digits) in [
        (12, vec![1, 2, 3, 4]),
        (2, vec![0, 2, 2]),
        (1, vec![6, 6, 6]),
        (0, vec![1, 3, 5]),
    ] {
        println!("* {digits:?}");
        assert_eq!(Sol3483::total_numbers(digits), rst);
        println!(":: {rst:?}");
    }
}
