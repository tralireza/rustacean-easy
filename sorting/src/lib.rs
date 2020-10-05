//! # Sorting

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
