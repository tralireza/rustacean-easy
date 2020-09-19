use super::*;

#[test]
fn test_1323() {
    for (rst, num) in [(9969, 9669), (9999, 9996), (9999, 9999)] {
        println!("* {num}");
        assert_eq!(Sol1323::maximum69_number(num), rst);
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
fn test_2413() {
    for (rst, n) in [(10, 5), (6, 6)] {
        println!("* {n}");
        assert_eq!(Sol2413::smallest_even_multiple(n), rst);
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
