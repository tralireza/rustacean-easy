//! # Sorting

/// 252 Meeting Rooms
struct Sol252 {}

impl Sol252 {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort();
        println!("-> {intervals:?}");

        intervals.windows(2).all(|w| w[0][1] <= w[1][0])
    }
}

/// 2229 Check if an Array is Consecutive
struct Sol2229 {}

impl Sol2229 {
    pub fn is_consecutive(mut nums: Vec<i32>) -> bool {
        nums.sort();

        nums.iter().zip(nums[0]..).filter(|&(&x, n)| x == n).count() == nums.len()
    }
}

/// 3667 Sort Array By Absolute Value
struct Sol3667 {}

impl Sol3667 {
    pub fn sort_by_absolute_value(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted: Vec<_> = nums.into_iter().collect();
        sorted.sort_by_key(|&n| n.abs());

        sorted
    }
}

/// 3684 Maximum Sum of At Most K Distinct Elements
struct Sol3684 {}

impl Sol3684 {
    pub fn max_k_distinct(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        {
            use std::cmp::Reverse;
            use std::collections::BTreeSet;

            let sorted: BTreeSet<Reverse<i32>> = nums.iter().map(|&n| Reverse(n)).collect();
            println!("-> BTreeSet: {sorted:?}");

            println!(
                ":? {:?}",
                sorted
                    .iter()
                    .take(k as usize)
                    .map(|Reverse(n)| n)
                    .collect::<Vec<_>>()
            );
        }

        nums.sort();
        nums.dedup();

        nums.into_iter().rev().take(k as usize).collect()
    }
}

#[cfg(test)]
mod tests;
