use super::*;

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
