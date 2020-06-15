use super::*;

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
