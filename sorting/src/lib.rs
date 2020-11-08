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

/// 3667 Sort Array By Absolute Value
struct Sol3667 {}

impl Sol3667 {
    pub fn sort_by_absolute_value(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted: Vec<_> = nums.into_iter().collect();
        sorted.sort_by_key(|&n| n.abs());

        sorted
    }
}

#[cfg(test)]
mod tests;
