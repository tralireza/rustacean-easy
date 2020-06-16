use super::*;

#[test]
fn test_2399() {
    for (rst, s, distance) in [
        (
            true,
            "abaccb",
            vec![
                1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ),
        (
            false,
            "aa",
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ),
    ] {
        println!("* {s:?} {distance:?}");
        assert_eq!(Sol2399::check_distances(s.to_string(), distance), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2409() {
    for (rst, arrive_alice, leave_alice, arrive_bob, leave_bob) in [
        (3, "08-15", "08-18", "08-16", "08-19"),
        (0, "10-01", "10-31", "11-01", "11-31"),
    ] {
        println!("*");
        assert_eq!(
            Sol2409::count_days_together(
                arrive_alice.to_string(),
                leave_alice.to_string(),
                arrive_bob.to_string(),
                leave_bob.to_string()
            ),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2423() {
    for (rst, word) in [
        (true, "abcc"),
        (false, "aazz"),
        (true, "bac"), // 34/50
    ] {
        println!("* {word:?}");
        assert_eq!(Sol2423::equal_frequency(word.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2437() {
    for (rst, time) in [(2, "?5:00"), (100, "0?:0?"), (1440, "??:??")] {
        println!("* {time:?}");
        assert_eq!(Sol2437::count_time(time.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2515() {
    for (rst, words, target, start_index) in [
        (1, vec!["hello", "i", "am", "leetcode", "hello"], "hello", 1),
        (1, vec!["a", "b", "leetcode"], "leetcode", 1),
        (-1, vec!["i", "eat", "leetcode"], "ate", 0),
    ] {
        let words: Vec<_> = words.iter().map(|w| w.to_string()).collect();
        println!("* {words:?} {target:?} {start_index}");
        assert_eq!(
            Sol2515::closest_target(words, target.to_string(), start_index),
            rst
        );
        println!(":: {rst:?}");
    }
}
