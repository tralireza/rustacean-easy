use super::*;

#[test]
fn test_2395() {
    for (rst, nums) in [
        (true, vec![4, 2, 4]),
        (false, vec![1, 2, 3, 4, 5]),
        (true, vec![0, 0, 0]),
        (false, vec![0, 0]), // 30/42
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2395::find_subarrays(nums), rst);
        println!(":: {rst:?}");
    }
}
