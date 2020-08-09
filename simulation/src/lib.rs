//! # Simulation

/// 3354 Make Array Element Equal to Zero
struct Sol3354 {}

impl Sol3354 {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let prefix_sum: Vec<_> = nums
            .iter()
            .scan(0, |csum, n| {
                *csum += n;
                Some(*csum)
            })
            .collect();

        let mut suffix_sum: Vec<_> = nums
            .iter()
            .rev()
            .scan(0, |csum, n| {
                *csum += n;
                Some(*csum)
            })
            .collect();
        suffix_sum.reverse();

        (0..nums.len())
            .filter(|&i| nums[i] == 0)
            .map(|i| {
                if prefix_sum[i] == suffix_sum[i] {
                    2
                } else if (prefix_sum[i] - suffix_sum[i]).abs() == 1 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

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
