use super::*;

#[test]
fn test_1085() {
    for (rst, nums) in [
        (0, vec![34, 23, 1, 24, 75, 33, 54, 8]),
        (1, vec![99, 77, 33, 66, 55]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol1085::sum_of_digits(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1056() {
    for (rst, n) in [(true, 6), (true, 89), (false, 11)] {
        println!("* {n}");
        assert_eq!(Sol1056::confusing_number(n), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1118() {
    for (rst, year, month) in [(31, 1992, 7), (29, 2000, 2), (28, 1900, 2)] {
        println!("* {year} {month}");
        assert_eq!(Sol1118::number_of_days(year, month), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1182() {
    for (rst, colors, queries) in [
        (
            vec![3, 0, 3],
            vec![1, 1, 2, 1, 3, 2, 2, 3, 3],
            vec![[1, 3], [2, 2], [6, 1]],
        ),
        (vec![-1], vec![1, 2], vec![[0, 3]]),
    ] {
        println!("* {colors:?} {queries:?}");
        assert_eq!(
            Sol1182::shortest_distance_color(
                colors,
                queries.into_iter().map(|a| Vec::from(a)).collect()
            ),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1228() {
    for (rst, arr) in [(9, vec![5, 7, 11, 13]), (14, vec![15, 13, 12])] {
        println!("* {arr:?}");
        assert_eq!(Sol1228::missing_number(arr), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1317() {
    for (rst, n) in [(vec![1, 1], 2), (vec![2, 9], 11)] {
        println!("* {n}");
        assert_eq!(Sol1317::get_no_zero_integers(n), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1323() {
    for (rst, num) in [(9969, 9669), (9999, 9996), (9999, 9999)] {
        println!("* {num}");
        assert_eq!(Sol1323::maximum69_number(num), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_1689() {
    for (rst, n) in [(3, "32"), (8, "82734"), (9, "27346209830709182346")] {
        println!("* {n}");
        assert_eq!(Sol1689::min_partitions(n.to_string()), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2348() {
    for (rst, nums) in [
        (6, vec![1, 3, 0, 0, 2, 0, 0, 4]),
        (9, vec![0, 0, 0, 2, 0, 0]),
        (0, vec![2, 10, 2019]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2348::zero_filled_subarray(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2396() {
    for (rst, n) in [(false, 9), (false, 4)] {
        println!("* {n}");
        assert_eq!(Sol2396::is_strictly_palindromic(n), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2413() {
    for (rst, n) in [(10, 5), (6, 6)] {
        println!("* {n}");
        assert_eq!(Sol2413::smallest_even_multiple(n), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2433() {
    for (rst, pref) in [
        (vec![5, 7, 2, 3, 2], vec![5, 2, 0, 3, 1]),
        (vec![13], vec![13]),
    ] {
        println!("* {pref:?}");
        assert_eq!(Sol2433::find_array(pref), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2525() {
    for (rst, length, width, height, mass) in
        [("Heavy", 1000, 35, 700, 300), ("Neither", 200, 50, 800, 50)]
    {
        println!("*");
        assert_eq!(Sol2525::categorize_box(length, width, height, mass), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2544() {
    for (rst, n) in [(4, 521), (1, 111), (0, 886996)] {
        println!("* {n}");
        assert_eq!(Sol2544::alternate_digit_sum(n), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2578() {
    for (rst, num) in [(59, 4325), (75, 687)] {
        println!("* {num}");
        assert_eq!(Sol2578::split_num(num), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2928() {
    for (rst, n, limit) in [(3, 5, 2), (10, 3, 3)] {
        println!("* {n} {limit}");
        assert_eq!(Sol2928::distribute_candies(n, limit), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2932() {
    for (rst, nums) in [
        (7, vec![1, 2, 3, 4, 5]),
        (0, vec![10, 100]),
        (7, vec![5, 6, 25, 30]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2932::maximum_strong_pair_xor(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3199() {
    for (rst, a, b, c) in [
        (1, vec![1], vec![2], vec![3]),
        (4, vec![1, 1], vec![2, 3], vec![1, 5]),
    ] {
        println!("* {a:?} {b:?} {c:?}");
        assert_eq!(Sol3199::triplet_count(a, b, c), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3658() {
    for (rst, n) in [(4, 4), (5, 5)] {
        println!("* {n}");
        assert_eq!(Sol3658::gcd_of_odd_even_sums(n), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3674() {
    for (rst, nums) in [(1, vec![1, 2]), (0, vec![5, 5, 5])] {
        println!("* {nums:?}");
        assert_eq!(Sol3674::min_operations(nums), rst);
        println!(":: {rst:?}");
    }
}
