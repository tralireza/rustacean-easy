use super::*;

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
