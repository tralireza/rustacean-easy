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
