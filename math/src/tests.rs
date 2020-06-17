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
