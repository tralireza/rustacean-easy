//! # Simulation

/// 3379 Transformed Array
struct Sol3379 {}

impl Sol3379 {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let m = nums.len();

        println!(
            ":? {:?}",
            nums.iter()
                .enumerate()
                .map(|(i, &n)| match n {
                    0.. => (i + n as usize) % m,
                    ..0 => (i as i32 + n % m as i32 + m as i32) as usize % m,
                })
                .map(|i| nums[i])
                .collect::<Vec<_>>()
        );

        nums.iter()
            .enumerate()
            .map(|(i, &n)| (i as i32 + n % m as i32 + m as i32) as usize % m)
            .map(|i| nums[i])
            .collect()
    }
}

#[cfg(test)]
mod tests;
