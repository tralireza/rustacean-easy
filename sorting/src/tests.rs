use super::*;

#[test]
fn test_252() {
    for (rst, intervals) in [
        (false, vec![[0, 30], [5, 10], [15, 20]]),
        (true, vec![[7, 10], [2, 4]]),
    ] {
        println!("* {intervals:?}");
        assert_eq!(
            Sol252::can_attend_meetings(
                intervals
                    .into_iter()
                    .map(|a| a.into_iter().collect())
                    .collect()
            ),
            rst
        );
        println!(":: {rst:?}");
    }
}

#[test]
fn test_2229() {
    for (rst, nums) in [
        (true, vec![1, 3, 4, 2]),
        (false, vec![1, 3]),
        (true, vec![3, 4, 5]),
        (false, vec![0, 3, 0, 3]), // 138/140
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol2229::is_consecutive(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3667() {
    for (rst, nums) in [
        (vec![-1, 1, 3, -4, 5], vec![3, -1, -4, 1, 5]),
        (vec![-100, 100], vec![-100, 100]),
    ] {
        println!("* {nums:?}");
        assert_eq!(Sol3667::sort_by_absolute_value(nums), rst);
        println!(":: {rst:?}");
    }
}

#[test]
fn test_3684() {
    for (rst, nums, k) in [
        (vec![100, 93, 90], vec![84, 93, 100, 77, 90], 3),
        (vec![100, 93, 84], vec![84, 93, 100, 77, 93], 3),
        (vec![2, 1], vec![1, 1, 1, 2, 2, 2], 6),
    ] {
        println!("* {nums:?} {k}");
        assert_eq!(Sol3684::max_k_distinct(nums, k), rst);
        println!(":: {rst:?}");
    }
}
