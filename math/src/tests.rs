use super::*;

#[test]
fn test_2413() {
    for (rst, n) in [(10, 5), (6, 6)] {
        println!("* {n}");
        assert_eq!(Sol2413::smallest_even_multiple(n), rst);
        println!(":: {rst:?}");
    }
}
