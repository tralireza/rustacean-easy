//! # Binary Search

/// 1099 Two Sum Less Than K
struct Sol1099 {}

impl Sol1099 {
    pub fn two_sum_less_than_k(mut nums: Vec<i32>, k: i32) -> i32 {
        {
            nums.sort();

            let mut xsum = -1;
            let (mut l, mut r) = (0, nums.len() - 1);
            while l < r {
                let cur_sum = nums[l] + nums[r];
                if cur_sum < k {
                    xsum = xsum.max(cur_sum);
                    l += 1;
                } else {
                    r -= 1;
                }
            }

            println!(":? O(N*log(N)) {xsum}");
        }

        nums.iter()
            .take(nums.len() - 1)
            .enumerate()
            .flat_map(|(i, x)| {
                nums.iter()
                    .skip(i + 1)
                    .filter(|&&y| x + y < k)
                    .map(|&y| y + x)
                    .max()
            })
            .max()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests;
