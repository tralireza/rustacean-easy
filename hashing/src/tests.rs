use super::*;

#[test]
fn test_1181() {
    for (rst, phrases) in [
        (
            vec!["writing code rocks"],
            vec!["writing code", "code rocks"],
        ),
        (
            vec![
                "a chip off the old block party",
                "a man on a mission impossible",
                "a man on a mission statement",
                "a quick bite to eat my words",
                "chocolate bar of soap",
            ],
            vec![
                "mission statement",
                "a quick bite to eat",
                "a chip off the old block",
                "chocolate bar",
                "mission impossible",
                "a man on a mission",
                "block party",
                "eat my words",
                "bar of soap",
            ],
        ),
        (vec!["a"], vec!["a", "b", "a"]),
    ] {
        let phrases: Vec<_> = phrases.iter().map(|p| p.to_string()).collect();

        println!("* {phrases:?}");
        assert_eq!(Sol1181::before_and_after_puzzles(phrases), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2395() {
    for (rst, nums) in [
        (true, vec![4, 2, 4]),
        (false, vec![1, 2, 3, 4, 5]),
        (true, vec![0, 0, 0]),
        (false, vec![0, 0]), // 30/42
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2395::find_subarrays(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2451() {
    for (rst, words) in [
        ("abc", vec!["adc", "wzy", "abc"]),
        ("bob", vec!["aaa", "bob", "ccc", "ddd"]),
    ] {
        let words: Vec<_> = words.iter().map(|w| w.to_string()).collect();

        println!("* {words:?}");
        assert_eq!(Sol2451::odd_string(words), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2500() {
    for (rst, grid) in [
        (8, vec![vec![1, 2, 4], vec![3, 3, 1]]),
        (10, vec![vec![10]]),
    ] {
        println!("* {grid:?}");
        assert_eq!(Sol2500::delete_greatest_value(grid), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2506() {
    for (rst, words) in [
        (2, vec!["aba", "aabb", "abcd", "bac", "aabc"]),
        (3, vec!["aabb", "ab", "ba"]),
        (0, vec!["nba", "cba", "dba"]),
    ] {
        let words: Vec<_> = words.iter().map(|w| w.to_string()).collect();
        println!("* {words:?}");
        assert_eq!(Sol2506::similar_pairs(words), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2682() {
    for (rst, n, k) in [(vec![4, 5], 5, 2), (vec![2, 3, 4], 4, 4)] {
        println!("* {n} {k}");
        assert_eq!(Sol2682::circular_game_losers(n, k), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2748() {
    for (rst, nums) in [(5, vec![2, 5, 1, 4]), (2, vec![11, 21, 12])] {
        println!("* {nums:?}");
        assert_eq!(Sol2748::count_beautiful_pairs(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2815() {
    for (rst, nums) in [
        (-1, vec![112, 131, 411]),
        (5902, vec![2536, 1613, 3366, 162]),
        (88, vec![51, 71, 17, 24, 42]),
        (165, vec![84, 91, 18, 59, 27, 9, 81, 33, 17, 58]), // 2512/3008
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2815::max_sum(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3032() {
    for (rst, a, b) in [(19, 1, 20), (10, 9, 19), (27, 80, 120)] {
        println!("* {a} {b}");
        assert_eq!(Sol3032::number_count(a, b), rst);
        println!(":: {rst:?}");
    }
}
