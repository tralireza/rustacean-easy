//! # Hashing

/// 2395 Find Subarrays With Equal Sum
struct Sol2395 {}

impl Sol2395 {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut sset = HashSet::new();
        nums.windows(2)
            .filter(|w| {
                if sset.contains(&w.iter().sum::<i32>()) {
                    true
                } else {
                    sset.insert(w.iter().sum::<i32>());
                    false
                }
            })
            .count()
            > 0
    }
}

#[cfg(test)]
mod tests;
