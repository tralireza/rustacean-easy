use super::*;

#[test]
fn test_1196() {
    for (rst, weight) in [
        (4, vec![100, 200, 150, 1000]),
        (5, vec![900, 950, 800, 1000, 700, 800]),
    ] {
        println!("* {weight:?}");
        assert_eq!(Sol1196::max_number_of_apples(weight), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2511() {
    for (rst, forts) in [
        (4, vec![1, 0, 0, -1, 0, 0, 0, 0, 1]),
        (0, vec![0, 0, 1, -1]),
        (1, vec![-1, -1, 0, 1, 0, 0, 1, -1, 1, 0]), // 42/45
    ] {
        println!("* {forts:?}");
        assert_eq!(Sol2511::capture_forts(forts), rst);
        println!(":: {rst:?}");
    }
}
