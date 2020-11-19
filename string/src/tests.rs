use super::*;

#[test]
fn test_245() {
    for (rst, words_dict, word1, word2) in [
        (
            3,
            vec!["practice", "makes", "perfect", "coding", "makes"],
            "coding",
            "practice",
        ),
        (
            1,
            vec!["practice", "makes", "perfect", "coding", "makes"],
            "makes",
            "coding",
        ),
    ] {
        println!("* {words_dict:?} {word1:?} {word2:?}");
        assert_eq!(
            Sol245::shortest_distance(
                words_dict.into_iter().map(|w| w.to_string()).collect(),
                word1.to_string(),
                word2.to_string()
            ),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_266() {
    for (rst, s) in [(false, "code"), (true, "aab"), (true, "carerac")] {
        println!("* {s:?}");
        assert_eq!(Sol266::can_permute_palindrome(s.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_293() {
    for (rst, current_state) in [(vec!["--++", "+--+", "++--"], "++++"), (vec![], "+")] {
        let rst: Vec<_> = rst.into_iter().map(|s| s.to_string()).collect();

        println!("* {current_state:?}");
        assert_eq!(
            Sol293::generate_possible_next_moves(current_state.to_string()),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_408() {
    for (rst, word, abbr) in [
        (true, "internationalization", "i12iz4n"),
        (false, "apple", "a2e"),
        (false, "hi", "1"), // 317/324
    ] {
        println!("* {word:?} {abbr:?}");
        assert_eq!(
            Sol408::valid_word_abbreviation(word.to_string(), abbr.to_string()),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_422() {
    for (rst, words) in [
        (true, vec!["abcd", "bnrt", "crmy", "dtye"]),
        (true, vec!["abcd", "bnrt", "crm", "dt"]),
        (false, vec!["ball", "area", "read", "lady"]),
    ] {
        println!("* {words:?}");
        assert_eq!(
            Sol422::valid_word_square(words.into_iter().map(|w| w.to_string()).collect()),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_734() {
    for (rst, sentence1, sentence2, similar_pairs) in [
        (
            true,
            vec!["great", "acting", "skills"],
            vec!["fine", "drama", "talent"],
            vec![["great", "fine"], ["drama", "acting"], ["skills", "talent"]],
        ),
        (true, vec!["great"], vec!["great"], vec![]),
        (
            false,
            vec!["great"],
            vec!["doubleplus", "great"],
            vec![["great", "doubleplus"]],
        ),
    ] {
        println!("* {sentence1:?} {sentence2:?} {similar_pairs:?}");
        assert_eq!(
            Sol734::are_sentences_similar(
                sentence1.into_iter().map(|s| s.to_string()).collect(),
                sentence2.into_iter().map(|s| s.to_string()).collect(),
                similar_pairs
                    .into_iter()
                    .map(|a| a.into_iter().map(|s| s.to_string()).collect())
                    .collect()
            ),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_800() {
    for (rst, color) in [("#11ee66", "#09f166"), ("#5544dd", "#4e3fe1")] {
        println!("* {color:?}");
        assert_eq!(Sol800::similar_rgb(color.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1065() {
    for (rst, text, words) in [
        (
            vec![[3, 7], [9, 13], [10, 17]],
            "thestoryofleetcodeandme",
            vec!["story", "fleet", "leetcode"],
        ),
        (
            vec![[0, 1], [0, 2], [2, 3], [2, 4]],
            "ababa",
            vec!["aba", "ab"],
        ),
    ] {
        let rst: Vec<Vec<_>> = rst.into_iter().map(|a| a.into_iter().collect()).collect();

        println!("* {text:?} {words:?}");
        assert_eq!(
            Sol1065::index_pairs(
                text.to_string(),
                words.into_iter().map(|s| s.to_string()).collect()
            ),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1180() {
    for (rst, s) in [(8, "aaaba"), (55, "aaaaaaaaaa")] {
        println!("* {s:?}");
        assert_eq!(Sol1180::count_letters(s.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1271() {
    for (rst, num) in [
        ("IOI", "257"),
        ("ERROR", "3"),
        ("AEIDBCDIBC", "747823223228"), // 344/393
    ] {
        println!("* {num:?}");
        assert_eq!(Sol1271::to_hexspeak(num.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1427() {
    for (rst, s, shift) in [
        ("cab", "abc", vec![[0, 1], [1, 2]]),
        ("efgabcd", "abcdefg", vec![[1, 1], [1, 1], [0, 2], [1, 3]]),
        (
            "qpifxqgwki",
            "xqgwkiqpif",
            vec![
                [1, 4],
                [0, 7],
                [0, 8],
                [0, 7],
                [0, 6],
                [1, 3],
                [0, 1],
                [1, 7],
                [0, 5],
                [0, 6],
            ],
        ), // 30/43
    ] {
        println!("* {s:?} {shift:?}");
        assert_eq!(
            Sol1427::string_shift(
                s.to_string(),
                shift.into_iter().map(|a| a.into_iter().collect()).collect()
            ),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1933() {
    for (rst, s) in [
        (false, "000111000"),
        (true, "00011111222"),
        (false, "011100022233"),
        (true, "66666666666677722"), // 88/122
        (false, "001"),              // 121/122
    ] {
        println!("* {s:?}");
        assert_eq!(Sol1933::is_decomposable(s.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1935() {
    for (rst, text, broken_letters) in [
        (1, "hello world", "ad"),
        (1, "leet code", "lt"),
        (0, "leet code", "e"),
    ] {
        println!("* {text:?} {broken_letters:?}");
        assert_eq!(
            Sol1935::can_be_typed_words(text.to_string(), broken_letters.to_string()),
            rst
        );
        println!(":: {rst:?}");
    }
}

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

#[test]
fn test_2609() {
    for (rst, s) in [(6, "01000111"), (4, "00111"), (0, "111")] {
        println!("* {s:?}");
        assert_eq!(
            Sol2609::find_the_longest_balanced_substring(s.to_string()),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2697() {
    for (rst, s) in [("efcfe", "egcfe"), ("abba", "abcd")] {
        println!("* {s:?}");
        assert_eq!(Sol2697::make_smallest_palindrome(s.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2744() {
    for (rst, words) in [
        (2, vec!["cd", "ac", "dc", "ca", "zz"]),
        (1, vec!["ab", "ba", "cc"]),
    ] {
        let words: Vec<_> = words.iter().map(|w| w.to_string()).collect();
        println!("* {words:?}");
        assert_eq!(Sol2744::maximum_number_of_string_pairs(words), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2788() {
    for (rst, words, separator) in [
        (
            vec!["one", "two", "three", "four", "five", "six"],
            vec!["one.two.three", "four.five", "six"],
            '.',
        ),
        (vec!["easy", "problem"], vec!["$easy$", "$problem$"], '$'),
        (vec![], vec!["|||"], '|'),
    ] {
        let words: Vec<_> = words.iter().map(|w| w.to_string()).collect();
        println!("* {words:?} {separator:?}");
        assert_eq!(Sol2788::split_words_by_separator(words, separator), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2833() {
    for (rst, moves) in [(3, "L_RL__R"), (5, "_R__LL_"), (7, "_______")] {
        println!("* {moves:?}");
        assert_eq!(
            Sol2833::furthest_distance_from_origin(moves.to_string()),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2839() {
    for (rst, s1, s2) in [(true, "abcd", "cdab"), (false, "abcd", "dacb")] {
        println!("* {s1:?} {s2:?}");
        assert_eq!(Sol2839::can_be_equal(s1.to_string(), s2.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3090() {
    for (rst, s) in [(4, "bcbbbcba"), (2, "aaaa")] {
        println!("* {s:?}");
        assert_eq!(Sol3090::maximum_length_substring(s.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3114() {
    for (rst, s) in [
        ("11:54", "1?:?4"),
        ("09:59", "0?:5?"),
        ("11:19", "??:1?"), // 662/908
        ("03:12", "?3:12"), // 759/908
    ] {
        println!("* {s:?}");
        assert_eq!(Sol3114::find_latest_time(s.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3280() {
    for (rst, date) in [
        ("100000100000-10-11101", "2080-02-29"),
        ("11101101100-1-1", "1900-01-01"),
    ] {
        println!("* {date:?}");
        assert_eq!(Sol3280::convert_date_to_binary(date.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3340() {
    for (rst, num) in [(false, "1234"), (true, "24123")] {
        println!("* {num:?}");
        assert_eq!(Sol3340::is_balanced(num.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3345() {
    for (rst, n, t) in [(10, 10, 2), (16, 15, 3)] {
        println!("* {n} {t}");
        assert_eq!(Sol3345::smallest_number(n, t), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3362() {
    for (rst, s, k) in [("dbb", "aadbbcccca", 3), ("xyz", "xyz", 2)] {
        println!("* {s:?} {k}");
        assert_eq!(Sol3362::filter_characters(s.to_string(), k), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3438() {
    for (rst, s) in [("23", "2523533"), ("21", "221"), ("", "22")] {
        println!("* {s:?}");
        assert_eq!(Sol3438::find_valid_pair(s.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3541() {
    for (rst, s) in [(6, "successes"), (3, "aeiaeia")] {
        println!("* {s:?}");
        assert_eq!(Sol3541::max_freq_sum(s.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3571() {
    for (rst, s1, s2) in [
        ("abab", "aba", "bab"),
        ("aaa", "aa", "aaa"),
        ("ix", "x", "ix"), // 1043/1153
    ] {
        println!("* {s1:?} {s2:?}");
        assert_eq!(
            Sol3571::shortest_superstring(s1.to_string(), s2.to_string()),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3602() {
    for (rst, n) in [("A91P1", 13), ("5101000", 36), ("F4240GJDGXS", 1000)] {
        println!("* {n}");
        assert_eq!(Sol3602::concat_hex36(n), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3606() {
    for (rst, code, business_line, is_active) in [
        (
            vec!["PHARMA5".to_string(), "SAVE20".to_string()],
            vec![
                "SAVE20".to_string(),
                "".to_string(),
                "PHARMA5".to_string(),
                "SAVE@20".to_string(),
            ],
            vec![
                "restaurant".to_string(),
                "grocery".to_string(),
                "pharmacy".to_string(),
                "restaurant".to_string(),
            ],
            vec![true, true, true, true],
        ),
        (
            vec!["ELECTRONICS_50".to_string()],
            vec![
                "GROCERY15".to_string(),
                "ELECTRONICS_50".to_string(),
                "DISCOUNT10".to_string(),
            ],
            vec![
                "grocery".to_string(),
                "electronics".to_string(),
                "invalid".to_string(),
            ],
            vec![false, true, true],
        ),
    ] {
        println!("* {code:?} {business_line:?} {is_active:?}");
        assert_eq!(
            Sol3606::validate_coupons(code, business_line, is_active),
            rst
        );
        println!(":: {rst:?}");
    }
}
